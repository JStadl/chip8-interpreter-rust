use crate::error::Chip8Error;
const MEMORY_SIZE_BYTES: usize = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Addr(u16);

impl Addr {
    pub const PROGRAM_START: Addr = Addr(0x200);
    pub const FONT_ZONE_START: Addr = Addr(0x050);

    pub fn new(raw: u16) -> Self {
        Addr(raw & 0x0FFF) // 12 bit addr invariant
    }

    pub fn offset(self, by: u16) -> Self {
        Addr::new(self.0.wrapping_add(by))
    }

    /// Like `offset`, but also reports whether the result went past 0xFFF.
    pub fn overflowing_offset(self, by: u16) -> (Self, bool) {
        let raw = self.0.wrapping_add(by);
        (Addr::new(raw), raw > 0x0FFF)
    }

    pub fn next_instruction(self) -> Self {
        self.offset(2)
    }

    pub fn prev_instruction(self) -> Self {
        Addr::new(self.0.wrapping_sub(2))
    }
}

pub struct Memory([u8; MEMORY_SIZE_BYTES]);

impl std::ops::IndexMut<Addr> for Memory {
    fn index_mut(&mut self, addr: Addr) -> &mut Self::Output {
        &mut self.0[addr.0 as usize]
    }
}

impl std::ops::Index<Addr> for Memory {
    type Output = u8;

    fn index(&self, addr: Addr) -> &Self::Output {
        &self.0[addr.0 as usize]
    }
}
impl Memory {
    pub fn new() -> Self {
        Memory([0; 4096])
    }

    // For loading roms and font etc, where wrapping etc would be a fault of the interpreter,
    // not the underlying program
    pub fn load(&mut self, start: Addr, data: &[u8]) -> Result<(), Chip8Error> {
        let begin = start.0 as usize;
        let end = begin + data.len();
        if end > self.0.len() {
            return Err(Chip8Error::MemoryOverflow {
                start: begin,
                len: data.len(),
            });
        }
        self.0[begin..end].copy_from_slice(data);
        Ok(())
    }
}
