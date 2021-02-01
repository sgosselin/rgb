#[derive(Copy, Clone)]
// TODO: document flags.
pub enum Flag {
    Z = 0b1000_0000,
    N = 0b0100_0000,
    H = 0b0010_0000,
    C = 0b0001_0000,
}

#[derive(Debug, Default)]
// Represents the LR35902's registers.
pub struct Regs {
    pub a: u8, pub f: u8,
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Regs {
    // Helper to decrement a 16-bit (2x 8-bit) registers.
    fn _dec_r16(msb: u8, lsb: u8) -> (u8, u8) {
        let res_lsb = u8::wrapping_sub(lsb, 1);
        let res_msb = if res_lsb == 0xff { u8::wrapping_sub(msb, 1) } else { msb };
        return (res_msb, res_lsb);
    }

    // Helper to increment a 16-bit (2x 8-bit) registers.
    fn _inc_r16(msb: u8, lsb: u8) -> (u8, u8) {
        let res_lsb = u8::wrapping_add(lsb, 1);
        let res_msb = if res_lsb == 0x00 { u8::wrapping_add(msb, 1) } else { msb };
        return (res_msb, res_lsb);
    }

    pub fn af(&self) -> u16 { ((self.a as u16) << 8) + (self.f as u16) }
    pub fn bc(&self) -> u16 { ((self.b as u16) << 8) + (self.c as u16) }
    pub fn de(&self) -> u16 { ((self.d as u16) << 8) + (self.e as u16) }
    pub fn hl(&self) -> u16 { ((self.h as u16) << 8) + (self.l as u16) }

    pub fn dec_bc(&mut self) {
        let (msb, lsb) = Regs::_dec_r16(self.b, self.c);
        self.b = msb; self.c = lsb;
    }

    pub fn dec_de(&mut self) {
        let (msb, lsb) = Regs::_dec_r16(self.d, self.e);
        self.d = msb; self.e = lsb;
    }

    pub fn dec_hl(&mut self) {
        let (msb, lsb) = Regs::_dec_r16(self.h, self.l);
        self.h = msb; self.l = lsb;
    }

    pub fn inc_hl(&mut self) {
        let (msb, lsb) = Regs::_inc_r16(self.h, self.l);
        self.h = msb; self.l = lsb;
    }

    pub fn set_bc(&mut self, raw: u16) {
        self.b = ((raw & 0xff00) >> 8) as u8;
        self.c = (raw & 0x00ff) as u8;
    }

    pub fn set_de(&mut self, raw: u16) {
        self.d = ((raw & 0xff00) >> 8) as u8;
        self.e = (raw & 0x00ff) as u8;
    }

    pub fn set_hl(&mut self, raw: u16) {
        self.h = ((raw & 0xff00) >> 8) as u8;
        self.l = (raw & 0x00ff) as u8;
    }

    pub fn set_flag(&mut self, flag: Flag, set: bool) {
        if set {
            self.f |= flag as u8;
        } else {
            self.f &= !(flag as u8);
        }
    }

    pub fn get_flag(&mut self, flag: Flag) -> bool {
        (self.f & (flag as u8)) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag() {
        let mut regs = Regs::default();

        regs.set_flag(Flag::Z, true);
        assert_eq!(regs.get_flag(Flag::Z), true);
        assert_eq!(regs.get_flag(Flag::N), false);
        assert_eq!(regs.get_flag(Flag::H), false);
        assert_eq!(regs.get_flag(Flag::C), false);
        regs.set_flag(Flag::Z, false);
        assert_eq!(regs.get_flag(Flag::Z), false);

        regs.set_flag(Flag::N, true);
        assert_eq!(regs.get_flag(Flag::Z), false);
        assert_eq!(regs.get_flag(Flag::N), true);
        assert_eq!(regs.get_flag(Flag::H), false);
        assert_eq!(regs.get_flag(Flag::C), false);
        regs.set_flag(Flag::N, false);
        assert_eq!(regs.get_flag(Flag::N), false);

        regs.set_flag(Flag::H, true);
        assert_eq!(regs.get_flag(Flag::Z), false);
        assert_eq!(regs.get_flag(Flag::N), false);
        assert_eq!(regs.get_flag(Flag::H), true);
        assert_eq!(regs.get_flag(Flag::C), false);
        regs.set_flag(Flag::H, false);
        assert_eq!(regs.get_flag(Flag::H), false);

        regs.set_flag(Flag::C, true);
        assert_eq!(regs.get_flag(Flag::Z), false);
        assert_eq!(regs.get_flag(Flag::N), false);
        assert_eq!(regs.get_flag(Flag::H), false);
        assert_eq!(regs.get_flag(Flag::C), true);
        regs.set_flag(Flag::C, false);
        assert_eq!(regs.get_flag(Flag::C), false);
    }

    #[test]
    fn test_reg16() {
        let mut regs = Regs::default();

        regs.set_bc(0x1020);
        assert_eq!(regs.b, 0x10);
        assert_eq!(regs.c, 0x20);

        regs.set_de(0x3040);
        assert_eq!(regs.d, 0x30);
        assert_eq!(regs.e, 0x40);

        regs.set_hl(0x5060);
        assert_eq!(regs.h, 0x50);
        assert_eq!(regs.l, 0x60);
    }

    #[test]
    fn test_dec_r16() {
        let mut regs = Regs::default();
        regs.set_bc(0x0100);
        regs.dec_bc();
        assert_eq!(regs.bc(), 0x00ff);

        regs = Regs::default();
        regs.set_de(0x0100);
        regs.dec_de();
        assert_eq!(regs.de(), 0x00ff);

        regs = Regs::default();
        regs.set_hl(0x0100);
        regs.dec_hl();
        assert_eq!(regs.hl(), 0x00ff);
    }

    #[test]
    fn test_inc_r16() {
        let mut regs = Regs::default();

        regs.set_hl(0x00ff);
        regs.inc_hl();
        assert_eq!(regs.hl(), 0x0100);
        regs.set_hl(0xffff);
        regs.inc_hl();
        assert_eq!(regs.hl(), 0x0000);
        regs.set_hl(0x0101);
        regs.inc_hl();
        assert_eq!(regs.hl(), 0x0102);
    }

}
