use crate::{Chip8, SCREEN_WIDTH, cpu::Instruction::*};
use crate::{Addr, SCREEN_HEIGHT};

enum Instruction {
    ClearScreen,                       // 00E0
    Return,                            // 00EE
    Jump(Addr),                        // 1NNN
    Call(Addr),                        // 2NNN
    JumpOffset(Addr),                  // BNNN  (jump to NNN + V0)

    Skip(SkipCond),                    // 3XNN 4XNN 5XY0 9XY0 EX9E EXA1

    SetImm { x: u8, value: u8 },       // 6XNN
    AddImm { x: u8, value: u8 },       // 7XNN
    Alu { op: AluOp, x: u8, y: u8 },   // 8XY0..8XYE
    Random { x: u8, mask: u8 },        // CXNN

    SetIndex(Addr),                    // ANNN
    AddIndex { x: u8 },                // FX1E
    FontChar { x: u8 },                // FX29  (point I at the sprite for digit VX)
    Bcd { x: u8 },                     // FX33
    Store { x: u8 },                   // FX55  (V0..=VX -> memory at I)
    Load { x: u8 },                    // FX65  (memory at I -> V0..=VX)

    Draw { x: u8, y: u8, n: u8 },      // DXYN

    ReadDelay { x: u8 },               // FX07
    SetDelay { x: u8 },                // FX15
    SetSound { x: u8 },                // FX18
    WaitKey { x: u8 },                 // FX0A
}

enum SkipCond {
    EqImm { x: u8, value: u8 },        // 3XNN  skip if VX == NN
    NeImm { x: u8, value: u8 },        // 4XNN  skip if VX != NN
    EqReg { x: u8, y: u8 },            // 5XY0  skip if VX == VY
    NeReg { x: u8, y: u8 },            // 9XY0  skip if VX != VY
    KeyDown { x: u8 },                 // EX9E  skip if key VX is pressed
    KeyUp { x: u8 },                   // EXA1  skip if key VX is not pressed
}

enum AluOp {
    Set,       // 8XY0  VX = VY
    Or,        // 8XY1
    And,       // 8XY2
    Xor,       // 8XY3
    Add,       // 8XY4  sets VF to carry
    Sub,       // 8XY5  VX = VX - VY, VF = no borrow
    ShiftRight,// 8XY6  quirk: shifts VY or VX
    SubN,      // 8XY7  VX = VY - VX, VF = no borrow
    ShiftLeft, // 8XYE  quirk: shifts VY or VX
}

impl Chip8 {
    pub fn step(&mut self) {
        let instr = Chip8::decode(self.fetch());
        self.execute(instr);
    }

    fn fetch(&mut self) ->  u16 {
        let ret = u16::from_be_bytes([self.memory[self.pc], self.memory[self.pc.offset(1)]]);
        self.pc = self.pc.offset(2);
        ret
    }

    fn decode(instr: u16) -> Instruction {
        let nibbles = (
            (instr >> 12) as u8,
            ((instr >> 8) & 0xF) as u8,
            ((instr >> 4) & 0xF) as u8,
            (instr & 0xF) as u8,
        );
        let (_, x, y, n) = nibbles;
        let nn = (instr & 0xFF) as u8;
        let nnn = Addr::new(instr); // Addr::new already masks to 12 bits

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => ClearScreen,
            (0x1, _, _, _) => Jump(nnn),
            (0x6, _, _, _) => SetImm{x, value: nn},
            (0x7, _, _, _) => AddImm { x, value: nn },
            (0xA, _, _, _) => SetIndex(nnn),
            (0xD, _, _, _) => Draw { x, y, n },
            _ => todo!()
        }
    }

    fn execute(&mut self, instr: Instruction) {
   
        match instr {
            ClearScreen => self.frame_buffer.clear_display(),
            Return => todo!(),
            Jump(addr) => self.pc = addr,
            Call(addr) => todo!(),
            JumpOffset(addr) => todo!(),
            Skip(skip_cond) => todo!(),
            SetImm { x, value } => self.registers[x as usize] = value,
            AddImm { x, value } => self.registers[x as usize] += value,
            Alu { op, x, y } => todo!(),
            Random { x, mask } => todo!(),
            SetIndex(addr) => self.i = addr,
            AddIndex { x } => todo!(),
            FontChar { x } => todo!(),
            Bcd { x } => todo!(),
            Store { x } => todo!(),
            Load { x } => todo!(),
            Draw { x, y, n } => {
                let x = self.registers[x as usize] % SCREEN_WIDTH as u8;
                let y = self.registers[y as usize] % SCREEN_HEIGHT as u8;
                
                self.registers[0xF] = 0;


                for i in 0..n {
                    // If off the bottom of the screen
                    if y + i >= SCREEN_HEIGHT as u8 {
                        break;
                    }
                    let row = self.memory[self.i.offset(i as u16)];
                    for bit_index in 0..8 {

                        // If gone off the right side
                        if x + bit_index >= SCREEN_WIDTH as u8 {
                            break;
                        }

                        if (row >> (7 - bit_index)) & 0x01 == 1 {
                            // flip
                            if self.frame_buffer.flip_pixel(x+bit_index, y+i) {
                                self.registers[0xF] = 1;
                            }
                        }
                    }  
                }


            },
            ReadDelay { x } => todo!(),
            SetDelay { x } => todo!(),
            SetSound { x } => todo!(),
            WaitKey { x } => todo!(),
        }
    }
}
