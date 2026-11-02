# Chip 8 Interpreter in Rust
Chip8 interpreter written in rust for my practice in a new language. Roms are used for testing, from https://github.com/Timendus/chip8-test-suite.
 
## Key layout: 
```
Q W E R    1 2 3 C
A S D F -> 4 5 6 D
Z X C V    A 0 B F
```
## Usage: 
```
cargo run --release -p chip8-frontend -- path/to/rom.ch8
```
![Pong](./pong.gif){  }