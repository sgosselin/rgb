use crate::cpu::{Flag, Opcode, Regs};
use crate::dbg::log;
use crate::mem::{Mmu};

#[derive(Default)]
/// Represents the LR35902 CPU (GameBoy's CPU).
pub struct Cpu {
    next_opcode_is_cb: bool,
    curr_opcode: Option<&'static Opcode>,
    regs: Regs,
}

impl Cpu {
    /// Create a new CPU object.
    pub fn new() -> Cpu {
        return Cpu {
            next_opcode_is_cb: false,
            curr_opcode: None,
            regs: Regs::default(),
        };
    }

    /// Steps the CPU through a fetch/decode/execute cycle.
    pub fn step(&mut self, mmu: &mut Mmu) -> usize {
        self.curr_opcode = Opcode::from(
            self.next_opcode_is_cb, self._fetch_next_byte(mmu));
        if self.curr_opcode.is_none() {
            self._panic("got an invalid opcode");
        }

        let res = if self.next_opcode_is_cb {
            self.next_opcode_is_cb = false;
            self._run_opcode_cb(mmu, self.curr_opcode.unwrap())
        } else {
            self._run_opcode_un(mmu, self.curr_opcode.unwrap())
        };

        log::info("cpu", "step", &format!("{:x?},  {:0x?})",
            self.curr_opcode.unwrap(), self.regs));

        return res;
    }

    // Fetch the next byte from PC and increase PC.
    fn _fetch_next_byte(&mut self, mmu: &Mmu) -> u8 {
        let res = mmu.read_byte(self.regs.pc);
        self.regs.pc = u16::wrapping_add(self.regs.pc, 1);
        return res;
    }

    // Fetch the next word from PC and increase PC.
    fn _fetch_next_word(&mut self, mmu: &Mmu) -> u16 {
        let res = mmu.read_word(self.regs.pc);
        self.regs.pc = u16::wrapping_add(self.regs.pc, 2);
        return res;
    }

    // Runs a cb-prefixed opcode.
    fn _run_opcode_cb(&mut self, mmu: &mut Mmu, opcode: &Opcode) -> usize {
        // Some instructions take a different number of cycles depending
        // on whether memory accesses were successful, or branches taken.
        // The dispatch code below will update the number of cycles if it
        // is different from the 'default' path. As such, we will have the
        // variable below mutable.
        let mut ncycles = opcode.ncycles.0;

        match opcode.x() {
            1 => { // BIT y, r[z]
                let r = self._get_r8_from_r(mmu, opcode.z());
                self.regs.set_flag(Flag::Z, (r & (1 << opcode.y())) == 0);
                self.regs.set_flag(Flag::N, false);
                self.regs.set_flag(Flag::H, true);
            },
            _ => {
                self._panic("cb-prefixed opcode not implemented");
            },
        };

        return ncycles;
    }

    // Runs a un-prefixed opcode.
    fn _run_opcode_un(&mut self, mmu: &mut Mmu, opcode: &Opcode) -> usize {
        // Some instructions take a different number of cycles depending
        // on whether memory accesses were successful, or branches taken.
        // The dispatch code below will update the number of cycles if it
        // is different from the 'default' path. As such, we will have the
        // variable below mutable.
        let mut ncycles = opcode.ncycles.0;

        match (opcode.x(), opcode.y(), opcode.z(), opcode.p(), opcode.q()) {
            (0, _, 1, _, 0) => { // LD rp[p], nn
                let nn = self._fetch_next_word(mmu);
                self._set_r16_from_rp(mmu, opcode.p(), nn);
            },
            (0, _, 2, 0, 1) => { // LD A,(BC)
                self.regs.a = mmu.read_byte(self.regs.bc());
            },
            (0, _, 2, 1, 1) => { // LD A,(DE)
                self.regs.a = mmu.read_byte(self.regs.de());
            },
            (0, _, 2, 2, 0) => { // LD (HL+), A
                mmu.write_byte(self.regs.hl(), self.regs.a);
                self.regs.inc_hl();
            },
            (0, _, 2, 3, 0) => { // LD (HL-), A
                mmu.write_byte(self.regs.hl(), self.regs.a);
                self.regs.dec_hl();
            },
            (0, _, 4, _, _) => { // INC r[y]
                let r = u8::wrapping_add(self._get_r8_from_r(mmu, opcode.y()), 1);
                self._set_r8_from_r(mmu, opcode.y(), r);
                self.regs.set_flag(Flag::Z, r == 0);
                self.regs.set_flag(Flag::N, false);
                self.regs.set_flag(Flag::H, (r & 0x0f) == 0x0f);
            },
            (0, _, 6, _, _) => { // LD r[y], n
                let n = self._fetch_next_byte(mmu);
                self._set_r8_from_r(mmu, opcode.y(), n);
            },
            (0, 4..=7, 0, _, _) => { // JR cc[y-4], d
                let d8 = self._fetch_next_byte(mmu) as i8;
                let pc = i32::wrapping_add(self.regs.pc as i32, d8 as i32) as u16;
                if self._get_res_from_cc(opcode.y() - 4) {
                    self.regs.pc = pc;
                } else {
                    ncycles = opcode.ncycles.1;
                }
            },
            (1, 6, 6, _, _) => { // HALT
                self._panic("HALT not implemented");
            },
            (1, _, _, _, _) => { // LD r[y], r[z]
                let r = self._get_r8_from_r(mmu, opcode.z());
                self._set_r8_from_r(mmu, opcode.y(), r);
            }
            (2, 5, _, _, _) => { // XOR r[z]
                self.regs.a ^= self._get_r8_from_r(mmu, opcode.z());
                self.regs.set_flag(Flag::Z, self.regs.a == 0);
                self.regs.set_flag(Flag::N, false);
                self.regs.set_flag(Flag::H, false);
                self.regs.set_flag(Flag::C, false);
            },
            (3, _, 5, 0, 1) => { // CALL nn
                let nn = self._fetch_next_word(mmu);
                self._stack_push(mmu, self.regs.pc);
                self.regs.pc = nn;
            },
            (3, _, 5, _, 0) => { // PUSH rp2[p]
                let nn = self._get_r16_from_rp2(mmu, opcode.p());
                self._stack_push(mmu, nn);
            },
            (3, 1, 3, _, _) => {
                self.next_opcode_is_cb = true;
            },
            (3, 4, 0, _, _) => { // LD (0xff00 + n),A
                let n = self._fetch_next_byte(mmu) as u16;
                mmu.write_byte(0xff00 + n, self.regs.a);
            },
            (3, 4, 2, _, _) => { // LD (0xff00 + C),A
                mmu.write_byte(0xff00 + (self.regs.c as u16), self.regs.a);
            },
            _ => {
                self._panic("un-prefixed opcode not implemented");
            },
        };

        return ncycles;
    }

    fn _get_res_from_cc(&self, cc: u8) -> bool {
        let res = match cc {
            0 => self.regs.get_flag(Flag::Z) == false,
            1 => self.regs.get_flag(Flag::Z) != false,
            2 => self.regs.get_flag(Flag::C) == false,
            3 => self.regs.get_flag(Flag::C) != false,
            _ => panic!("impossible <cc> index"),
        };

        return res;
    }

    fn _get_r8_from_r(&self, mmu: &Mmu, r: u8) -> u8 {
        return match r {
            0 => self.regs.b,
            1 => self.regs.c,
            2 => self.regs.d,
            3 => self.regs.e,
            4 => self.regs.h,
            5 => self.regs.l,
            6 => mmu.read_byte(self.regs.hl()),
            7 => self.regs.a,
            _ => panic!("impossible <r> index"),
        };
    }

    fn _get_r16_from_rp2(&mut self, mmu: &mut Mmu, rp2: u8) -> u16 {
        return match rp2 {
            0 => self.regs.bc(),
            1 => self.regs.de(),
            2 => self.regs.hl(),
            3 => self.regs.af(),
            _ => panic!("impossible <rp2> index")
        };
    }

    fn _set_r8_from_r(&mut self, mmu: &mut Mmu, r: u8, val: u8) {
        match r {
            0 => self.regs.b = val,
            1 => self.regs.c = val,
            2 => self.regs.d = val,
            3 => self.regs.e = val,
            4 => self.regs.h = val,
            5 => self.regs.l = val,
            6 => mmu.write_byte(self.regs.hl(), val),
            7 => self.regs.a = val,
            _ => panic!("impossible <r> index"),
        };
    }

    fn _set_r16_from_rp(&mut self, mmu: &mut Mmu, rp: u8, val: u16) {
        match rp {
            0 => self.regs.set_bc(val),
            1 => self.regs.set_de(val),
            2 => self.regs.set_hl(val),
            3 => self.regs.sp = val,
            _ => self._panic("impossible <rp> index"),
        };
    }

    fn _set_r16_from_rp2(&mut self, mmu: &mut Mmu, rp2: u8, val: u16) {
        match rp2 {
            0 => self.regs.set_bc(val),
            1 => self.regs.set_de(val),
            2 => self.regs.set_hl(val),
            3 => self.regs.set_af(val),
            _ => panic!("impossible <rp2> index")
        };
    }

    fn _stack_push(&mut self, mmu: &mut Mmu, word: u16) {
        self.regs.sp = u16::wrapping_sub(self.regs.sp, 2);
        mmu.write_word(self.regs.sp, word);
    }

    fn _stack_pop(&mut self, mmu: &mut Mmu) -> u16 {
        let res = mmu.read_word(self.regs.sp);
        self.regs.sp = u16::wrapping_add(self.regs.sp, 2);
        return res;
    }

    // Dumps the CPU state and exits.
    fn _panic(&self, reason: &str) -> ! {
        println!("=============== cpu panic ===============");
        println!("regs: {:#0x?}", self.regs);
        println!("-----------------------------------------");
        println!("next_opcode_is_cb: {}", self.next_opcode_is_cb);
        println!("-----------------------------------------");
        println!("curr_opcode: {:#x?}", self.curr_opcode);
        println!("=============== cpu panic ===============");
        panic!("panic reason: {}", reason);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: write tests for each opcodes.
}
