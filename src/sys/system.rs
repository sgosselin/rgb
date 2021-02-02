use crate::cpu::{Cpu};
use crate::mem::{Mmu};

pub struct System {
    pub cpu: Cpu,
    pub mmu: Mmu,
}

impl System {

    // TODO: add a function to load a cartridge.

    pub fn new() -> System {
        return System {
            cpu: Cpu::new(),
            mmu: Mmu::new(),
        };
    }

    pub fn step(&mut self) {
        let ncycles = self.cpu.step(&mut self.mmu);
        self.mmu.gpu.step(ncycles);
    }
}
