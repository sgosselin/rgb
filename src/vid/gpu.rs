use crate::dbg::log;

/// Enumerates the possible mode for the GPU.
///
/// ## Description of each modes
///
/// Mode 2: OAM Scan
/// TODO
///
/// Mode 3: Drawing
/// TODO
/// Mode 0: HBlank
/// TODO
///
/// Mode 1: VBlank
/// TODO
///
///
/// ## State diagram of the modes and their transitions:
///
///            +-------------------------+
///            |                         +No
///  +---------v--------+        XXXXXXXXXXXXXXX  Yes    +-----------------------+
///  |  Mode 1: VBlank  +------>X  LY == 153 ?  X+--+--->+   Mode 2: OAM Search  +<--+
///  +---------+--------+        XXXXXXXXXXXXXXX    ^    +------------+----------+   |
///            ^                                    |                 |              |
///            |                                    |                 v              |
///            +Yes                                 |            XXXXXXXXXX  No      |
///     XXXXXXXXXXXXXXX  No                         |           X  Done ?  X+--------+
///    X  LY == 144 ?  X+---------------------------+            XXXXXXXXXX
///     XXXXXXXXXXXXXXX                                               +Yes
///            ^                                                      |
///            |                                                      v
///            +Yes                                        +----------+-----------+
///        XXXXXXXXXX  No                                  |    Pixel Transfer    +<--+
///       X  Done ?  X+---------+                          +----------+-----------+   |
///        XXXXXXXXXX           |                                     |               |
///            ^                |                                     v               |
///            |                |                             XXXXXXXXXXXXXXX  No     |
/// +----------+------------+   |                            X  LX == 160 ?  X+-------+
/// |    Mode 0: HBlank     +<--+                             XXXXXXXXXXXXXXX
/// +----------+------------+                                         +Yes
///            ^                                                      |
///            +------------------------------------------------------+
///
#[derive(Copy, Clone)]
enum Mode {
    HBlank,     // Mode 0
    VBlank,     // Mode 1
    OamScan,    // Mode 2
    Drawing,    // Mode 3
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
            curr_mode: Mode::OamScan,
        };
    }

    /// Steps the GPU for a certain number of cycles.
    pub fn step(&mut self, ncycles: usize) {
        // TODO.

        log::info("gpu", "step", &format!("ncycles={}", ncycles));
    }


    /// Copies the screen region of the internal framebuffer.
    pub fn copy_screen(&self, dst: &[u8]) {
        // TODO
    }
}
