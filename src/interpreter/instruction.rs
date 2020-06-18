use std::fmt::{Display, Formatter, Result};

type Addr = u16;
type Vx = usize;
type Vy = usize;
type Byte = u8;
type Nib = u8;

// lsb = least significant bit

// ErrUnsupportedInstruction is thrown when an unsupported instruction was used.
pub struct ErrUnsupportedInstruction;

impl std::fmt::Display for ErrUnsupportedInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "that's an unsupported instruction")
    }
}

#[derive(Debug)]
pub enum Instruction {
    I0NNN(Addr),        // 0NNN
    I00E0,              // 00E0
    I00EE,              // 00EE
    I1NNN(Addr),        // 1NNN
    I2NNN(Addr),        // 2NNN
    I3XNN(Vx, Byte),    // 3XNN
    I4XNN(Vx, Byte),    // 4XNN
    I5XY0(Vx, Vy),      // 5XY0
    I6XNN(Vx, Byte),    // 6XNN
    I7XNN(Vx, Byte),    // 7XNN
    I8XY0(Vx, Vy),      // 8XY0
    I8XY1(Vx, Vy),      // 8XY1
    I8XY2(Vx, Vy),      // 8XY2
    I8XY3(Vx, Vy),      // 8XY3
    I8XY4(Vx, Vy),      // 8XY4
    I8XY5(Vx, Vy),      // 8XY5
    I8XY6(Vx, Vy),      // 8XY6
    I8XY7(Vx, Vy),      // 8XY7
    I8XYE(Vx, Vy),      // 8XYE
    I9XY0(Vx, Vy),      // 9XY0
    IANNN(Addr),        // ANNN
    IBNNN(Addr),        // BNNN
    ICXNN(Vx, Byte),    // CXNN
    IDXYN(Vx, Vy, Nib), // DXYN
    IEX9E(Vx),          // EX9E
    IEXA1(Vx),          // EXA1
    IFX07(Vx),          // FX07
    IFX0A(Vx),          // FX0A
    IFX15(Vx),          // FX15
    IFX18(Vx),          // FX18
    IFX1E(Vx),          // FX1E
    IFX29(Vx),          // FX29
    IFX33(Vx),          // FX33
    IFX55(Vx),          // FX55
    IFX65(Vx),          // FX65

    UNSUPPORTED, // Represents an unsupported instruction
}

impl Instruction {
    pub fn get_instr_from_parts(
        opcode: u16,
        jmp: u16,
        x: usize,
        y: usize,
        nn: u16,
    ) -> Self {
        return match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => Instruction::I00E0,
                0x00EE => Instruction::I00EE,
                _ => Instruction::UNSUPPORTED,
            },
            0x1000 => Instruction::I1NNN(jmp),
            0x2000 => Instruction::I2NNN(jmp),
            0x3000 => Instruction::I3XNN(x, nn as u8),
            0x4000 => Instruction::I4XNN(x, nn as u8),
            0x5000 => Instruction::I5XY0(x, y),
            0x6000 => Instruction::I6XNN(x, nn as u8),
            0x7000 => Instruction::I7XNN(x, nn as u8),
            0x8000 => match opcode & 0x000F {
                0x0000 => Instruction::I8XY0(x, y),
                0x0001 => Instruction::I8XY1(x, y),
                0x0002 => Instruction::I8XY2(x, y),
                0x0003 => Instruction::I8XY3(x, y),
                0x0004 => Instruction::I8XY4(x, y),
                0x0005 => Instruction::I8XY5(x, y),
                0x0006 => Instruction::I8XY6(x, y),
                0x0007 => Instruction::I8XY7(x, y),
                0x000E => Instruction::I8XYE(x, y),
                _ => Instruction::UNSUPPORTED,
            },
            0x9000 => Instruction::I9XY0(x, y),
            0xA000 => Instruction::IANNN(jmp),
            0xB000 => Instruction::IBNNN(jmp),
            0xC000 => Instruction::ICXNN(x, nn as u8),
            0xD000 => Instruction::IDXYN(x, y, (opcode & 0x000F) as u8),
            0xE000 => match opcode & 0x000F {
                0x000E => Instruction::IEX9E(x),
                0x0001 => Instruction::IEXA1(x),
                _ => Instruction::UNSUPPORTED,
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => Instruction::IFX07(x),
                0x000A => Instruction::IFX0A(x),
                0x0015 => Instruction::IFX15(x),
                0x0018 => Instruction::IFX18(x),
                0x001E => Instruction::IFX1E(x),
                0x0029 => Instruction::IFX29(x),
                0x0033 => Instruction::IFX33(x),
                0x0055 => Instruction::IFX55(x),
                0x0065 => Instruction::IFX65(x),
                _ => Instruction::UNSUPPORTED,
            },
            _ => Instruction::UNSUPPORTED,
        };
    }
}
