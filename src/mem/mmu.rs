use crate::dbg::log;
use crate::vid::{Gpu};
use super::bios::{BIOS};

// BIOS:
//
// When the GameBoy boots up, the BIOS is mapped in memory. After its
// execution, the BIOS is unmapped and can't be remapped. The BIOS is
// contained in src/mem/bios.rs, with comments.
//
const BIOS_BEG_ADDR: u16 = 0x0000;
const BIOS_END_ADDR: u16 = 0x00FF;


const VRAM_BEG_ADDR: u16 = 0x8000;
const VRAM_END_ADDR: u16 = 0x9fff;
const VRAM_LEN:usize = (VRAM_END_ADDR - VRAM_BEG_ADDR + 1) as usize;

// Working RAM:
//
// This region represents the internal memory of the GameBoy. This is
// used by all games, especially those without memory banks on their
// cartridge.
//
const WRAM_BEG_ADDR: u16 = 0xc000;
const WRAM_END_ADDR: u16 = 0xdfff;
const WRAM_LEN:usize = (WRAM_END_ADDR - WRAM_BEG_ADDR + 1) as usize;

// Reserved RAM:
//
// This region echoes the Working RAM region. However, Nintendo recommended
// that nobody uses this region. We still need to emulate this region since
// some games did not listen to Nintendo.
//
const RRAM_BEG_ADDR: u16 = 0xe000;
const RRAM_END_ADDR: u16 = 0xfdff;
const RRAM_LEN: usize = (RRAM_END_ADDR - RRAM_BEG_ADDR + 1) as usize;

// Zero RAM:
//
// Originally intended to be used as stack space, it is also used for
// fast memory access since some instructions operating in the 0xFF00
// to 0xFFFF range (e.g., LD (C), A) are fsaster than typical LD instrs.
//
const ZRAM_BEG_ADDR: u16 = 0xff80;
const ZRAM_END_ADDR: u16 = 0xfffe;
const ZRAM_LEN: usize = (ZRAM_END_ADDR - ZRAM_BEG_ADDR + 1) as usize;

/// Represents the memory interconnect of the GameBoy.
///
/// In order to simplify the design, the memory interconnect (here, MMU) contains the
/// memory mapped I/O devices. The GameBoy contains multiple I/O devices, such as the
/// PPU/GPU.
///
/// Each I/O device has its memory mapped into the address space. They are usually
/// registers specific to this device. Writes & reads operating on the I/O devices
/// are delegated to each device accordingly.
///
pub struct Mmu {
    is_bios_mapped: bool,
    vram: [u8; VRAM_LEN],
    wram: [u8; WRAM_LEN],
    zram: [u8; ZRAM_LEN],

    pub gpu: Gpu,
}

impl Mmu {
    /// Creates an initialized Mmu.
    pub fn new() -> Mmu {
        return Mmu {
            is_bios_mapped: true,
            vram: [0x00; VRAM_LEN],
            wram: [0x00; WRAM_LEN],
            zram: [0x00; ZRAM_LEN],
            gpu: Gpu::new(),
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

    /// Reads a word from memory at |addr|; returns 0x00 if the
    /// memory region is unmapped.
    pub fn read_byte(&self, addr: u16) -> u8 {
        let res = match (self.is_bios_mapped, addr) {
            (true, BIOS_BEG_ADDR..=BIOS_END_ADDR) => {
                BIOS[(addr - BIOS_BEG_ADDR) as usize]
            },
            (_, VRAM_BEG_ADDR..=VRAM_END_ADDR) => {
                self.vram[(addr - VRAM_BEG_ADDR) as usize]
            },
            (_, WRAM_BEG_ADDR..=WRAM_END_ADDR) => {
                self.wram[(addr - WRAM_BEG_ADDR) as usize]
            },
            (_, RRAM_BEG_ADDR..=RRAM_END_ADDR) => {
                self.wram[(addr - RRAM_BEG_ADDR) as usize]
            },
            (_, ZRAM_BEG_ADDR..=ZRAM_END_ADDR) => {
                self.zram[(addr - ZRAM_BEG_ADDR) as usize]
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

    /// Reads a word from memory at |addr|; returns 0x0000 if the
    /// memory region is unmapped.
    pub fn read_word(&self, addr: u16) -> u16 {
        let lsb = self.read_byte(addr) as u16;
        let msb = self.read_byte(addr + 1) as u16;

        return (msb << 8) | lsb;
    }

    /// Writes |d8| into memory at |addr|.
    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            VRAM_BEG_ADDR..=VRAM_END_ADDR => {
                self.vram[(addr - VRAM_BEG_ADDR) as usize] = val;
            },
            WRAM_BEG_ADDR..=WRAM_END_ADDR => {
                self.wram[(addr - WRAM_BEG_ADDR) as usize] = val;
            },
            RRAM_BEG_ADDR..=RRAM_END_ADDR => {
                self.wram[(addr - RRAM_BEG_ADDR) as usize] = val;
            }
            ZRAM_BEG_ADDR..=ZRAM_END_ADDR => {
                self.zram[(addr - ZRAM_BEG_ADDR) as usize] = val;
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

    #[test]
    fn test_wram() {
        let mut mmu = Mmu::new();
        for addr in WRAM_BEG_ADDR..=WRAM_END_ADDR {
            mmu.write_byte(addr, 0x10);
            assert_eq!(mmu.read_byte(addr), 0x10);
        }
    }

    #[test]
    fn test_rram() {
        let mut mmu = Mmu::new();
        for rram_addr in RRAM_BEG_ADDR..=RRAM_END_ADDR {
            mmu.write_byte(rram_addr, 0x10);
            // Calculate the corresponding address in wram; verify RRAM
            // echoes WRAM.
            let wram_addr = WRAM_BEG_ADDR + (rram_addr - RRAM_BEG_ADDR);
            assert_eq!(mmu.read_byte(wram_addr), 0x10);
        }
    }

    #[test]
    fn test_zram() {
        let mut mmu = Mmu::new();
        for addr in ZRAM_BEG_ADDR..=ZRAM_END_ADDR {
            mmu.write_byte(addr, 0x10);
            assert_eq!(mmu.read_byte(addr), 0x10);
        }
    }
}
