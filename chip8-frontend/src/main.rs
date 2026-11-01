use core::panic;

use chip8_core::{Chip8, FrameBuffer, SCREEN_HEIGHT, SCREEN_WIDTH};
use minifb::{Window, WindowOptions};

const PIXEL_ON_VAL:u32 = u32::MAX;
const PIXEL_OFF_VAL:u32 = 0;

fn redraw(fb: &FrameBuffer, window: &mut Window) {
    let buf = fb.pixels().map(|x| if x {PIXEL_ON_VAL} else {PIXEL_OFF_VAL} );
    if window.update_with_buffer(&buf, SCREEN_WIDTH, SCREEN_HEIGHT).is_err() {
        // TODO figure out if this is handleable
        panic!("Updating buffer error");
    }
}

fn main() {
    let mut chip8: Chip8 = Chip8::default();
    if chip8.load_rom("./roms/2-ibm-logo.ch8").is_err() {
        // TODO let user specify another
        panic!("Couldn't load rom, try another");
    }
    let mut window = match Window::new(
        "Chip8",
        64,
        32,
        WindowOptions {
            scale: minifb::Scale::X16,
            topmost: true,
            ..Default::default()
        },
    ) {
        Ok(win) => win,
        Err(err) => {
            panic!("Unable to create window {}", err)
        }
    };
    println!("Hello, world!");
    while window.is_open() {
        // TODO OPTIMIZE: for now, redraw every step even if no display change
        redraw(chip8.framebuffer(), &mut window);
        chip8.step();

    }
}
