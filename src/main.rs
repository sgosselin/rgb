use clap::{Arg, App};
use minifb::{Key, Window, WindowOptions};

// Include these modules here so sub-modules can find them via crate::.  I am not
// sure whether there is a cleaner way to do this.
mod cpu;
mod dbg;
mod mem;
mod sys;
mod vid;

fn app_gui() {
    let win_w = vid::gpu::SCREEN_W;
    let win_h = vid::gpu::SCREEN_H;
    let mut buffer: Vec<u32> = vec![0; win_w * win_h];
    let mut sys = sys::System::new();


    let mut window = Window::new(".: RGB - GameBoy Emulator :. (ESC to exit)",
            win_w, win_h, WindowOptions::default())
        .unwrap_or_else(|e| { panic!("{}", e); });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        sys.mmu.gpu.copy_screen(&mut buffer[..]);
        window.update_with_buffer(&buffer, win_w, win_h).unwrap();
    }
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
        .arg(Arg::with_name("gui")
            .long("gui")
            .multiple(false)
            .help("start a gui version of the emulator"))
        .get_matches();

    if matches.is_present("gui") {
        app_gui();
    } else {
        app_cli();
    }
}
