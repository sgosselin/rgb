use clap::{Arg, App};

// Include these modules here so sub-modules can find them via crate::.  I am not
// sure whether there is a cleaner way to do this.
mod cpu;
mod dbg;
mod mem;
mod sys;
mod vid;

fn app_tui() {
    // TODO.
}

fn app_cli() {
    let mut sys = sys::System::new();

    loop {
        sys.step();
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
