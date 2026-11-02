use crate::{Chip8, SCREEN_WIDTH, cpu::Instruction::*};
use crate::{Addr, SCREEN_HEIGHT, font};

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
    pub fn is_beep(&self) -> bool {
        self.sound_timer > 0
    }

    pub fn tick_timers(&mut self) {
        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
        self.waiting_for_vblank = false;

    }
    pub fn step(&mut self, key_state: &[bool; 16]) {
        if self.waiting_for_vblank {
            return;
        }
        let instr = Chip8::decode(self.fetch());
        self.execute(instr, key_state);
        self.prev_key_state = *key_state;
    }

    fn fetch(&mut self) ->  u16 {
        let ret = u16::from_be_bytes([self.memory[self.pc], self.memory[self.pc.offset(1)]]);
        self.pc = self.pc.next_instruction();
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
            (0x0, 0x0, 0xE, 0xE) => Return,
            (0x1, _, _, _) => Jump(nnn),
            (0x2, _, _, _) => Call(nnn),
            (0xB, _, _, _) => JumpOffset(nnn),

            (0x3, _, _, _) => Skip(SkipCond::EqImm { x, value: nn }),
            (0x4, _, _, _) => Skip(SkipCond::NeImm { x, value: nn }),
            (0x5, _, _, 0x0) => Skip(SkipCond::EqReg { x, y }),
            (0x9, _, _, 0x0) => Skip(SkipCond::NeReg { x, y }),
            (0xE, _, 0x9, 0xE) => Skip(SkipCond::KeyDown { x }),
            (0xE, _, 0xA, 0x1) => Skip(SkipCond::KeyUp { x }),

            (0x6, _, _, _) => SetImm{x, value: nn},
            (0x7, _, _, _) => AddImm { x, value: nn },

            (0x8, _, _, 0x0) => Alu { op: AluOp::Set, x, y },
            (0x8, _, _, 0x1) => Alu { op: AluOp::Or, x, y },
            (0x8, _, _, 0x2) => Alu { op: AluOp::And, x, y },
            (0x8, _, _, 0x3) => Alu { op: AluOp::Xor, x, y },
            (0x8, _, _, 0x4) => Alu { op: AluOp::Add, x, y },
            (0x8, _, _, 0x5) => Alu { op: AluOp::Sub, x, y },
            (0x8, _, _, 0x6) => Alu { op: AluOp::ShiftRight, x, y },
            (0x8, _, _, 0x7) => Alu { op: AluOp::SubN, x, y },
            (0x8, _, _, 0xE) => Alu { op: AluOp::ShiftLeft, x, y },

            (0xC, _, _, _) => Random { x, mask: nn },

            (0xA, _, _, _) => SetIndex(nnn),
            (0xF, _, 0x1, 0xE) => AddIndex{x},
            (0xF, _, 0x2, 0x9) => FontChar{x},
            (0xF, _, 0x3, 0x3) => Bcd{x},
            (0xF, _, 0x5, 0x5) => Store{x},
            (0xF, _, 0x6, 0x5) => Load{x},

            (0xD, _, _, _) => Draw { x, y, n },

            (0xF, _, 0x0, 0x7) => ReadDelay{x},
            (0xF, _, 0x1, 0x5) => SetDelay{x},
            (0xF, _, 0x1, 0x8) => SetSound{x},
            (0xF, _, 0x0, 0xA) => WaitKey{x},
            _ => {
                panic!("Couldn't find this instruction definition")
            }
        }
    }

    fn execute(&mut self, instr: Instruction, key_state: &[bool; 16]) {
   
        match instr {
            ClearScreen => self.frame_buffer.clear_display(),
            Return => self.pc = self.stack.pop().expect("Stack is empty"),
            Jump(addr) => self.pc = addr,
            Call(addr) => {
                self.stack.push(self.pc);
                self.pc = addr
            },
            JumpOffset(addr) => {
                self.pc = addr + Addr::new(self.registers[0x0].into());
            },
            Skip(skip_cond) => {
                let skip = match skip_cond {
                    SkipCond::EqImm { x, value } => self.registers[x as usize] == value,
                    SkipCond::NeImm { x, value } => !(self.registers[x as usize] == value),
                    SkipCond::EqReg { x, y } => self.registers[x as usize] == self.registers[y as usize],
                    SkipCond::NeReg { x, y } => !(self.registers[x as usize] == self.registers[y as usize]),
                    SkipCond::KeyDown { x } => key_state[self.registers[x as usize] as usize & 0xF],
                    SkipCond::KeyUp { x } => !(key_state[self.registers[x as usize] as usize & 0xF]),
                };
                if skip {
                    self.pc = self.pc.next_instruction();
                }
            },
            SetImm { x, value } => self.registers[x as usize] = value,
            AddImm { x, value } => self.registers[x as usize] = self.registers[x as usize].wrapping_add(value),
            Alu { op, x, y } => {
                match op {
                    AluOp::Set => self.registers[x as usize] = self.registers[y as usize],
                    AluOp::Or => {
                        self.registers[x as usize] |= self.registers[y as usize];
                        self.registers[0xF] = 0;
                    },
                    AluOp::And => {
                        self.registers[x as usize] &= self.registers[y as usize];
                        self.registers[0xF] = 0;
                    },
                    AluOp::Xor => {
                        self.registers[x as usize] ^= self.registers[y as usize];
                        self.registers[0xF] = 0;
                    },
                    AluOp::Add => {
                        let (val, overflowed) = self.registers[x as usize].overflowing_add(self.registers[y as usize]);
                        self.registers[x as usize] = val;
                        self.registers[0xF] = overflowed as u8;
                    },
                    AluOp::Sub => {
                        let (val, overflowed) = self.registers[x as usize].overflowing_sub(self.registers[y as usize]);
                        self.registers[x as usize] = val;
                        self.registers[0xF] = !overflowed as u8;
                    },
                    AluOp::SubN => {
                        let (val, overflowed) = self.registers[y as usize].overflowing_sub(self.registers[x as usize]);
                        self.registers[x as usize] = val;
                        self.registers[0xF] = !overflowed as u8;
                    },
                    AluOp::ShiftRight => {
                        self.registers[x as usize] = self.registers[y as usize];
                        let flag: u8 = if self.registers[x as usize] & 0x1 == 1 {
                            1
                        } else {
                            0
                        };
                        self.registers[x as usize] >>= 1;
                        self.registers[0xF] = flag;

                    },
                    AluOp::ShiftLeft => {
                        self.registers[x as usize] = self.registers[y as usize];
                        let flag = if (self.registers[x as usize] >> 7) & 0x1 == 1 {
                            1
                        } else {
                            0
                        };
                        self.registers[x as usize] <<= 1;
                        self.registers[0xF] = flag;

                    },
                }
            },
            Random { x, mask } => {
                self.registers[x as usize] = rand::random_range(0..u8::MAX) & mask;
            },
            SetIndex(addr) => self.i = addr,
            AddIndex { x } => {
                let (new_i, overflowed) = self.i.overflowing_offset(u16::from(self.registers[x as usize]));
                self.i = new_i;
                self.registers[0xF] = overflowed as u8;
            },
            FontChar { x } => {
                let char = x & 0x0F;
                self.i = Addr::FONT_ZONE_START.offset(char as u16 * font::FONT_CHAR_SIZE_BYTES);
            },
            Bcd { x } => {
                let num = self.registers[x as usize];
                let units = num % 10;
                let tens = (num % 100 - units) / 10;
                let hundreds = (num - units - tens) / 100;
                self.memory[self.i] = hundreds;
                self.memory[self.i.offset(1)] = tens;
                self.memory[self.i.offset(2)] = units;
            },
            Store { x } => {
                for i in 0..x+1 {
                    self.memory[self.i] = self.registers[i as usize];
                    self.i = self.i.offset(1);
                }
            },
            Load { x } => {
                for i in 0..x+1 {
                    self.registers[i as usize] = self.memory[self.i];
                    self.i = self.i.offset(1);

                }
            },
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
                self.waiting_for_vblank = true;

            },
            ReadDelay { x } => self.registers[x as usize] = self.delay_timer,
            SetDelay { x } => self.delay_timer = self.registers[x as usize],
            SetSound { x } => self.sound_timer = self.registers[x as usize],
            WaitKey { x } => {
                for (i, val) in key_state.iter().enumerate() {
                    if !self.prev_key_state[i] && *val {
                        self.registers[x as usize] = i as u8;
                        return;
                    }
                }
                self.pc = self.pc.prev_instruction();

            },
        }
    }
}
