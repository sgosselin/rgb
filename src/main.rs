use clap::{Arg, App};

// Include these modules here so sub-modules can find them via crate::.  I am not
// sure whether there is a cleaner way to do this.
mod cpu;
mod dbg;
mod mem;
mod vid;

struct MyApp {
    cpu: cpu::Cpu,
    mmu: mem::Mmu,
    is_running: bool,
}

impl MyApp {
    pub fn new() -> MyApp {
        return MyApp {
            cpu: cpu::Cpu::new(),
            mmu: mem::Mmu::new(),
            is_running: true,
        };
    }

    pub fn step(&mut self) {
        let ncycles = self.cpu.step(&mut self.mmu);
        self.mmu.gpu.step(ncycles);
    }
}

fn app_tui() {
    let mut app = MyApp::new();
    // TODO.
}

fn app_cli() {
    let mut app = MyApp::new();

    while app.is_running {
        app.step();
    }
}

fn main() {
    let matches = App::new("rgb")
        .version("0.1.0")
        .author("Samuel Gosselin")
        .arg(Arg::with_name("tui")
            .long("tui")
            .multiple(false)
            .help("start a terminal based ui"))
        .get_matches();

    if matches.is_present("tui") {
        app_tui();
    } else {
        app_cli();
    }
}
