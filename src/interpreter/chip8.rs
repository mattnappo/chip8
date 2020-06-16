use super::instruction::{ErrUnsupportedInstruction, Instruction};
use crate::arithmetic;
use rand::Rng;
use std::convert::TryInto;
use std::error;
use std::fs;

const MEM_SIZE: usize = 0x1000;
const N_REGISTERS: usize = 16;
const STACK_DEPTH: usize = 12;
const PROGRAM_START: u16 = 0x200;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const N_KEYS: usize = 16;
const F: usize = 0xF;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10,
    0xF0, 0x80, 0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10,
    0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0,
    0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80,
    0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

const OFF: u8 = 0x0;
const ON: u8 = 0x1;

// Chip8 is the struct that represents a single CHIP-8 interpreter.
pub struct Chip8 {
    pub memory: [u8; MEM_SIZE], // The memory
    V: [u8; N_REGISTERS],       // The general purpose registers
    I: u16,                     // The I register
    stack: [u8; STACK_DEPTH],   // The stack

    pc: u16, // The program counter
    sp: u8,  // The stack pointer

    delay_timer: u16, // The delay timer
    sound_timer: u16, // The sound timer

    display: [u8; WIDTH * HEIGHT], // The display
    keys: [u8; N_KEYS],            // The keys (include here?)
}

impl Chip8 {
    // new constructs a new CHIP-8 interpreter.
    pub fn new() -> Self {
        let mut c8 = Self {
            memory: [0; MEM_SIZE],
            V: [0; N_REGISTERS],
            I: 0,
            stack: [0; STACK_DEPTH],

            pc: PROGRAM_START,
            sp: 0,

            delay_timer: 0,
            sound_timer: 0,

            display: [OFF; WIDTH * HEIGHT],
            keys: [0; N_KEYS], // Not actually initialized like this
        };
        c8.install_fontset();
        c8
    }

    // install_fontset loads the font ROM into memory.
    fn install_fontset(&mut self) {
        for i in 0..FONTSET.len() {
            self.memory[i] = FONTSET[i];
        }
    }

    // memory_dump prints a certain amount of bytes of the system memory.
    pub fn memory_dump(&self, n_bytes: usize) {
        let chars_per_line = 200;
        for i in 0..n_bytes {
            for j in 0..chars_per_line {
                print!("{:x}", self.memory[i]);
            }
            println!("");
        }
    }

    // register_dump prints the registers of the system.
    pub fn register_dump(&self) {
        for i in 0..self.V.len() {
            println!("V{} = {:x}", i, self.V[i]);
        }
    }
    // parse_file will parse a CHIP-8 source code file and load it into the machine.
    pub fn parse_file(&mut self, filename: &str) {}

    // load_rom loads a ROM given the filename of the ROM and loads it into the machine.
    pub fn load_rom(&mut self, filename: &str) -> Result<(), std::io::Error> {
        let rom = fs::read(filename)?;
        for i in 0..rom.len() {
            self.memory[i + PROGRAM_START as usize] = rom[i];
        }
        Ok(())
    }

    // execute executes the virtual machine as it is in its current state.
    pub fn execute(&mut self) {
        while self.pc < self.memory.len() as u16 {
            println!("program counter: {}", self.pc);
            // The only reason why these are u16s is because it will make them easier
            // to deal with when determining the instruction.
            let current_instr: u16 = self.memory[self.pc as usize].into();
            let next_instr: u16 = self.memory[self.pc as usize + 1].into();
            let opcode: u16 = (current_instr << 8) | next_instr;
            let jmp: u16 = (opcode & 0x0FF).into(); // NNN (the jump address)
            let x: usize = ((opcode & 0x0F00) >> 8).into();
            let y: usize = ((opcode & 0x00F0) >> 4).into();
            let nn: u16 = opcode & 0x00FF;

            let instr =
                Instruction::get_instr_from_parts(opcode, jmp, x, y, nn);
            self.execute_instruction(instr);
        }
    }

    // execute_instruction executes a single instruction.
    pub fn execute_instruction(
        &mut self,
        i: Instruction,
    ) -> Result<(), ErrUnsupportedInstruction> {
        let mut should_jump = false;
        match i {
            Instruction::I0NNN(a) => {} // Not really implemented
            Instruction::I00E0 => {
                // Clear the display
                for i in 0..self.display.len() {
                    self.display[i] = OFF;
                }
            }
            Instruction::I00EE => {
                // Return from subroutine
                if self.stack[self.sp as usize] == 0 {
                    panic!("no value to return to on the stack");
                }
                self.pc = self.stack[self.sp as usize].into(); // Pop off the stack
                self.stack[self.sp as usize] = 0; // Remove the value
                if self.sp > 0 {
                    self.sp -= 1; // Update the stack pointer
                }
            }
            Instruction::I1NNN(a) => {
                // Jump to address NNN
                self.pc = a;
                should_jump = true;
            }
            Instruction::I2NNN(a) => {
                // Call suborutine at address NNN
                if self.stack[self.sp as usize] != 0 {
                    self.sp += 1;
                }
                self.stack[self.sp as usize] = self.pc.try_into().unwrap(); // Push current address to the stack
                self.pc = a;
                should_jump = true;
            }
            Instruction::I3XNN(x, b) => {}
            Instruction::I4XNN(x, b) => {}
            Instruction::I5XY0(x, y) => {}
            Instruction::I6XNN(x, b) => self.V[x] = b,
            Instruction::I7XNN(x, b) => self.V[x] += b,
            Instruction::I8XY0(x, y) => self.V[x] = self.V[y],
            Instruction::I8XY1(x, y) => self.V[x] |= self.V[y],
            Instruction::I8XY2(x, y) => self.V[x] &= self.V[y],
            Instruction::I8XY3(x, y) => self.V[x] ^= self.V[y],
            Instruction::I8XY4(x, y) => {
                self.V[F] = arithmetic::check_carry(&self.V[x], &self.V[y]);
                self.V[x] += self.V[y];
            }
            Instruction::I8XY5(x, y) => {
                self.V[F] = arithmetic::check_borrow(&self.V[x], &self.V[y]);
                self.V[x] -= self.V[y];
            }
            Instruction::I8XY6(x, y) => {
                self.V[F] = arithmetic::get_lsb(&self.V[y]);
                self.V[x] = self.V[y] >> 1;
            }
            Instruction::I8XY7(x, y) => {
                self.V[F] = arithmetic::check_borrow(&self.V[x], &self.V[y]);
                self.V[x] = self.V[y] - self.V[x];
            }
            Instruction::I8XYE(x, y) => {
                self.V[F] = arithmetic::get_msb(&self.V[y]);
                self.V[x] = self.V[y] << 1;
            }
            Instruction::I9XY0(x, y) => {}
            Instruction::IANNN(a) => {}
            Instruction::IBNNN(a) => self.pc = a + (self.V[0] as u16),
            Instruction::ICXNN(x, b) => {
                let r: u8 = rand::thread_rng().gen();
                self.V[x] = r & b;
            }
            Instruction::IDXYN(x, y, n) => {}
            Instruction::IEX9E(x) => {}
            Instruction::IEXA1(x) => {}
            Instruction::IFX07(x) => {}
            Instruction::IFX0A(x) => {}
            Instruction::IFX15(x) => {}
            Instruction::IFX18(x) => {}
            Instruction::IFX1E(x) => {}
            Instruction::IFX29(x) => {}
            Instruction::IFX33(x) => {}
            Instruction::IFX55(x) => {}
            Instruction::IFX65(x) => {}

            Instruction::UNSUPPORTED => return Err(ErrUnsupportedInstruction),
        };

        if !should_jump {
            self.pc += 2;
        }
        Ok(())
    }
}
