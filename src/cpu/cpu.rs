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
        self._panic("cb-prefixed opcode not implemented");
        0
    }

    // Runs a un-prefixed opcode.
    fn _run_opcode_un(&mut self, mmu: &mut Mmu, opcode: &Opcode) -> usize {
        // Some instructions take a different number of cycles depending
        // on whether memory accesses were successful, or branches taken.
        // The dispatch code below will update the number of cycles if it
        // is different from the 'default' path. As such, we will have the
        // variable below mutable.
        let mut ncycles = opcode.ncycles.0;

        let res = match (opcode.x(), opcode.y(), opcode.z(), opcode.p(), opcode.q()) {
            (3, 1, 3, _, _) => {
                self.next_opcode_is_cb = true;
            },
            (0, _, 1, _, 0) => { // LD rp[p], nn
                let nn = self._fetch_next_word(mmu);
                self._set_r16_from_rp(mmu, opcode.p(), nn);
            },
            (2, 5, _, _, _) => { // XOR r[z]
                self.regs.a ^= self._get_r8_from_r(mmu, opcode.z());
                self.regs.set_flag(Flag::Z, self.regs.a == 0);
                self.regs.set_flag(Flag::N, false);
                self.regs.set_flag(Flag::H, false);
                self.regs.set_flag(Flag::C, false);
            },
            _ => {
                self._panic("un-prefixed opcode not implemented");
            },
        };

        return ncycles;
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

    fn _set_r16_from_rp(&mut self, mmu: &mut Mmu, rp: u8, val: u16) {
        match rp {
            0 => self.regs.set_bc(val),
            1 => self.regs.set_de(val),
            2 => self.regs.set_hl(val),
            3 => self.regs.sp = val,
            _ => self._panic("impossible <rp> index"),
        };
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

    // TODO: write tests for cpu.
}
