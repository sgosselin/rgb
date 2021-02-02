/// Enumerates the possible mode for the GPU.
///
/// Mode0: TODO
/// Mode1: TODO
/// Mode2: TODO
/// Mode3: TODO
#[derive(Copy, Clone)]
enum Mode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

/// Represents the GameBoy's GPU; also known as PPU for
/// Pixel Processing Unit.  It is responsible for rendering
/// sprites onto the framebuffer.
pub struct Gpu {
    curr_mode: Mode,
}

impl Gpu {

    /// Creates a new GPU object.
    pub fn new() -> Gpu {
        return Gpu {
            curr_mode: Mode::Mode0,
        };
    }
}
