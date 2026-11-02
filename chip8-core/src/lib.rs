mod cpu;
mod display;
mod error;
mod font;
mod memory;

use std::path::Path;

pub use display::{FrameBuffer, SCREEN_HEIGHT, SCREEN_WIDTH};
pub use error::Chip8Error;

use memory::{Addr, Memory};

const NUM_REGISTERS: usize = 16;

/// Struct representing the state of the Chip8 interpreter
pub struct Chip8 {
    memory: Memory,
    frame_buffer: FrameBuffer,
    pc: Addr,
    i: Addr,
    stack: Vec<Addr>,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; NUM_REGISTERS],
    prev_key_state: [bool; 16],
    waiting_for_vblank: bool,
}
impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            memory: Memory::new(),
            frame_buffer: FrameBuffer::default(),
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            pc: Addr::PROGRAM_START,
            i: Addr::new(0),
            registers: [0; NUM_REGISTERS],
            prev_key_state: [false; 16],
            waiting_for_vblank: false,
        };
        chip8.load_font();
        chip8
    }

    fn load_font(&mut self) {
        self.memory
            .load(Addr::FONT_ZONE_START, &crate::font::FONT)
            .expect("font always fits in memory");
    }

    pub fn framebuffer(&self) -> &FrameBuffer {
        &self.frame_buffer
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Chip8Error> {
        let rom = std::fs::read(path)?;
        self.memory.load(Addr::PROGRAM_START, &rom)
    }

    
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
