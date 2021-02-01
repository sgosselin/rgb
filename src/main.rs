// TODO: include these modules here so sub-modules can find them
// via crate::.  I'm not sure whether there is a cleaner way to do
// this.
mod cpu;
mod dbg;
mod mem;
mod vid;

fn main() {
    let mut cpu = cpu::Cpu::new();
    let mut mmu = mem::Mmu::new();
    cpu.step(&mut mmu);
}
