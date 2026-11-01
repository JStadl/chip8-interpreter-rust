use std::fmt;
#[derive(Debug)]
pub enum Chip8Error {
    Io(std::io::Error),
    MemoryOverflow { start: usize, len: usize },
}

impl fmt::Display for Chip8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Chip8Error::Io(e) => write!(f, "I/O error: {e}"),
            Chip8Error::MemoryOverflow { start, len } => {
                write!(f, "{len} bytes at {start:#05X} would overflow memory")
            }
        }
    }
}

impl std::error::Error for Chip8Error {
    // Lets callers see the underlying I/O error
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Chip8Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

// This is what makes `?` work on io::Result inside functions returning Chip8Error
impl From<std::io::Error> for Chip8Error {
    fn from(e: std::io::Error) -> Self {
        Chip8Error::Io(e)
    }
}
