use core::panic;
use std::{thread::sleep, time::{Duration, Instant}};

use chip8_core::{Chip8, FrameBuffer, SCREEN_HEIGHT, SCREEN_WIDTH};
use minifb::{Key, Window, WindowOptions};

const PIXEL_ON_VAL:u32 = u32::MAX;
const PIXEL_OFF_VAL:u32 = 0;

fn redraw(fb: &FrameBuffer, window: &mut Window) {
    let buf = fb.pixels().map(|x| if x {PIXEL_ON_VAL} else {PIXEL_OFF_VAL} );
    if window.update_with_buffer(&buf, SCREEN_WIDTH, SCREEN_HEIGHT).is_err() {
        // TODO figure out if this is handleable
        panic!("Updating buffer error");
    }
}

const KEYMAP: [Key; 16] = [
    Key::X,                    // 0
    Key::Key1, Key::Key2, Key::Key3, // 1 2 3
    Key::Q, Key::W, Key::E,    // 4 5 6
    Key::A, Key::S, Key::D,    // 7 8 9
    Key::Z, Key::C,            // A B
    Key::Key4, Key::R, Key::F, Key::V, // C D E F
];

fn keypad_state(window: &Window) -> [bool; 16] {
    KEYMAP.map(|k| window.is_key_down(k))
}

fn main() {
    let mut chip8: Chip8 = Chip8::default();
    if chip8.load_rom("./roms/6-keypad.ch8").is_err() {
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
    let refresh_rate = 60.0;
    let steps_per_refresh: f64 = 700.0 / refresh_rate;
    let interval = Duration::from_secs_f64(1.0/refresh_rate);
    let mut next_time = Instant::now() + interval;
    while window.is_open() {
        
        redraw(chip8.framebuffer(), &mut window);
        chip8.tick_timers();
        if chip8.is_beep() {
            actually_beep::beep_with_hz_and_millis(329, (1.0/refresh_rate) as u32).unwrap();
        }
        for _ in 0..steps_per_refresh as i64 {
            chip8.step(&keypad_state(&window));
        }
        sleep(next_time - Instant::now());
        next_time += interval;
    }
}
