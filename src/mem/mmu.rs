use crate::dbg::log;
use super::bios::{BIOS};

const BIOS_BEG_ADDR: u16 = 0x0000;
const BIOS_END_ADDR: u16 = 0x00FF;

const WRAM_BEG_ADDR: u16 = 0xc000;
const WRAM_END_ADDR: u16 = 0xdfff;
const WRAM_LEN:usize = (WRAM_END_ADDR - WRAM_BEG_ADDR + 1) as usize;

pub struct Mmu {
    is_bios_mapped: bool,
    wram: [u8; WRAM_LEN],
}

impl Mmu {
    /// Creates an initialized Mmu.
    pub fn new() -> Mmu {
        return Mmu {
            is_bios_mapped: true,
            wram: [0x00; WRAM_LEN],
        };
    }

    /// Returns |true| iff. the BIOS is mapped.
    pub fn is_bios_mapped(&self) -> bool {
        self.is_bios_mapped
    }

    /// Removes the BIOS from memory.
    pub fn unmap_bios(&mut self) {
        self.is_bios_mapped = false;
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        let res = match (self.is_bios_mapped, addr) {
            (true, BIOS_BEG_ADDR..=BIOS_END_ADDR) => {
                BIOS[(addr - BIOS_BEG_ADDR) as usize]
            },
            (_, WRAM_BEG_ADDR..=WRAM_END_ADDR) => {
                self.wram[(addr - WRAM_BEG_ADDR) as usize]
            },
            _ => {
                // The GameBoy returns 0x00 when nothing can be read at
                // a specific address.  This manifests itself when the
                // device boots without a cartridge: the NINTENDO logo
                // is entirely black.
                0x00
            },
        };

        log::info("mmu", "read_byte", &format!("addr=0x{:04x} res=0x{:02x}",
            addr, res));

        return res;
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let lsb = self.read_byte(addr) as u16;
        let msb = self.read_byte(addr + 1) as u16;

        return (msb << 8) | lsb;
    }

    /// Writes |d8| into memory at |addr|.
    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            WRAM_BEG_ADDR..=WRAM_END_ADDR => {
                self.wram[(addr - WRAM_BEG_ADDR) as usize] = val;
            },
            _ => {
                /* NOP */
            },
        }
    }

    /// Writes |d16| into memory at |addr|.
    pub fn write_word(&mut self, addr: u16, d16: u16) {
        let lsb = (d16 & 0x00ff) as u8;
        let msb = ((d16 >> 8) & 0x00ff) as u8;
        self.write_byte(addr, lsb);
        self.write_byte(addr + 1, msb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_byte() {
        let mmu = Mmu::new();

        assert_eq!(mmu.read_byte(0x00), 0x31); 
    }

    #[test]
    fn test_read_word() {
        let mmu = Mmu::new();

        assert_eq!(mmu.read_word(0x01), 0xfffe); 
    }

    #[test]
    fn test_bios() {
        let mut mmu = Mmu::new();

        assert_eq!(mmu.is_bios_mapped(), true);
        assert_eq!(mmu.read_byte(0x00), 0x31); 

        mmu.unmap_bios();
        assert_eq!(mmu.read_byte(0x00), 0x00);
    }

    #[test]
    fn test_write_read_byte() {
        let mut mmu = Mmu::new();

        mmu.write_byte(WRAM_BEG_ADDR, 0x42);
        assert_eq!(mmu.read_byte(WRAM_BEG_ADDR), 0x42);

        // BIOS is not writtable.
        mmu.write_byte(BIOS_BEG_ADDR, 0x42);
        assert_ne!(mmu.read_byte(BIOS_BEG_ADDR), 0x42);
    }

    #[test]
    fn test_write_read_word() {
        let mut mmu = Mmu::new();

        mmu.write_word(WRAM_BEG_ADDR, 0x1020);
        assert_eq!(mmu.read_word(WRAM_BEG_ADDR), 0x1020);
    }

}
