pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub struct FrameBuffer {
    raw_buffer: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}
impl Default for FrameBuffer {
    fn default() -> Self {
        Self {
            raw_buffer: [false; SCREEN_HEIGHT * SCREEN_WIDTH],
        }
    }
}

impl FrameBuffer {
    pub fn clear_display(&mut self) {
        self.raw_buffer.fill(false);
    }
    pub fn pixels(&self) -> &[bool; SCREEN_HEIGHT * SCREEN_WIDTH] {
        &self.raw_buffer
    }
}
