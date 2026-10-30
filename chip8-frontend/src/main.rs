use core::panic;

use chip8_core::Chip8;
use minifb::{Window, WindowOptions};

fn main() {
    
    let mut chip8: Chip8 = Chip8::default();
    let fb: &[[bool; 32]; 64] = chip8.framebuffer();
    if let Err(_) = chip8.load_rom("./roms/2-ibm-logo.ch8") {
        panic!("Couldn't load rom");
    }
    let mut window = match Window::new(
        "Chip8",
        64,
        32, 
        WindowOptions { 
            scale: minifb::Scale::X16, 
            ..Default::default()
        }) {
        Ok(win) => win,
        Err(err) => {
            panic!("Unable to create window {}", err)
        }
    };
    println!("Hello, world!");
    while window.is_open() {
        window.update();
    }
}
