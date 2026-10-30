use std::{collections::VecDeque, fs::{self, File}, io::{Error, Read}, path::Path};

const FONT : [u8; 80]  = [0xF0, 0x90, 0x90, 0x90, 0xF0, 
                          0x20, 0x60, 0x20, 0x20, 0x70, 
                          0xF0, 0x10, 0xF0, 0x80, 0xF0, 
                          0xF0, 0x10, 0xF0, 0x10, 0xF0, 
                          0x90, 0x90, 0xF0, 0x10, 0x10, 
                          0xF0, 0x80, 0xF0, 0x10, 0xF0, 
                          0xF0, 0x80, 0xF0, 0x90, 0xF0, 
                          0xF0, 0x10, 0x20, 0x40, 0x40, 
                          0xF0, 0x90, 0xF0, 0x90, 0xF0, 
                          0xF0, 0x90, 0xF0, 0x10, 0xF0, 
                          0xF0, 0x90, 0xF0, 0x90, 0x90, 
                          0xE0, 0x90, 0xE0, 0x90, 0xE0, 
                          0xF0, 0x80, 0x80, 0x80, 0xF0, 
                          0xE0, 0x90, 0x90, 0x90, 0xE0, 
                          0xF0, 0x80, 0xF0, 0x80, 0xF0, 
                          0xF0, 0x80, 0xF0, 0x80, 0x80];
const PROGRAM_START: usize = 0x200;

type FrameBuffer = [[bool; 32]; 64];

/// Struct representing the state of the Chip8 interpreter
pub struct Chip8 {
    memory: [u8; 4096],
    frame_buffer: [[bool; 32]; 64],
    stack: VecDeque<u16>,
    delay_timer: u8,
    sound_timer: u8,
}
impl Default for Chip8 {
    fn default() -> Self {
        Chip8::new()
    }
}

impl Chip8 {
    fn new() -> Self {
        let mut chip8 = Chip8 { memory: [0; 4096], frame_buffer: [[false;32]; 64], stack: VecDeque::new(), delay_timer: 0, sound_timer: 0 };
        chip8.load_font();
        chip8
    }

    fn load_font(&mut self) {
        self.memory[0x050..0x09F+1].copy_from_slice(&FONT);
    }

    pub fn framebuffer(&self) -> &[[bool; 32]; 64] {
        &self.frame_buffer
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, path : P) -> std::io::Result<()> {
        let mut rom = File::open(path)?;
        let max = self.memory.len() - PROGRAM_START;
        let mut buf= Vec::new();
        let n_read_bytes = rom.read_to_end(&mut buf)?;
        if n_read_bytes > max {
            return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("ROM is {} bytes; max is {max}", n_read_bytes),
        ));
        }
        self.memory[PROGRAM_START..PROGRAM_START+n_read_bytes].copy_from_slice(&buf);
        Ok(())
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
