use crate::dbg::log;

pub const SCREEN_W: usize = 160;
pub const SCREEN_H: usize = 144;

/// Enumerates the possible mode for the GPU.
///
/// ## Description of each modes
///
/// Mode 2: OAM Scan
/// TODO
///
/// Mode 3: Drawing
/// TODO
///
/// Mode 0: HBlank
/// TODO
///
/// Mode 1: VBlank
/// TODO
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
///        XXXXXXXXXX  No                                  |    Mode 3: Drawing   +<--+
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
/// ## Timing diagram of the modes for one frame
///
/// Mode 2  2_____2_____2_____2_____2_____2___________________2____
/// Mode 3  _33____33____33____33____33____33__________________3___
/// Mode 0  ___000___000___000___000___000___000________________000
/// Mode 1  ____________________________________11111111111111_____
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


    /// Renders the framebuffer into a screen region, according to the selected
    /// palette.  Note the |dst| buffer is a 0RGB.  The upper 8-bits are ignored,
    /// the next 8-bits are for the red channel, the next 8-bits afterwards for
    /// the green channel, and the lower 8-bits for the blue channel.
    pub fn copy_screen(&self, dst: &mut [u32]) {
        assert_eq!(dst.len(), SCREEN_W * SCREEN_H);

        // TODO: renders something real.
        for y in 0..SCREEN_H {
            for x in 0..SCREEN_W {
                let r:u32 = (x as u32) % 255;
                let ind = y * SCREEN_W + x;
                dst[ind] = r << 16;
            }
        }
    }
}
