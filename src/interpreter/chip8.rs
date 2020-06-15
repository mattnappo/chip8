use super::instruction::Instruction as Instr;
use crate::arithmetic;
use rand::Rng;
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
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

enum Pixel {
    ON,
    OFF,
}

impl Pixel {
    fn get_bit(&self) -> u8 {
        match self {
            ON => 1,
            OFF => 0,
        }
    }
}

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

    display: [Pixel; WIDTH * HEIGHT], // The display
    keys: [u8; N_KEYS],               // The keys (include here?)
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

            display: [Pixel::OFF; WIDTH * HEIGHT],
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
            let opcode = self.memory[self.pc] << 8 | self.memory[self.pc + 1];
            let jmp = opcode & 0x0FF;
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            let nn = opcode & 0x00FF;

            let instr = Instr::get_instr_from_parts(opcode, jmp, x, y, nn);
            self.execute_instruction(instr);
        }
    }

    // execute_instruction executes a single instruction.
    pub fn execute_instruction(&mut self, i: Instr) {
        match i {
            Instr::I0NNN(a) => {}
            Instr::I00E0 => {}
            Instr::I00EE => {}
            Instr::I1NNN(a) => self.pc = a,
            Instr::I2NNN(a) => {}
            Instr::I3XNN(x, b) => {}
            Instr::I4XNN(x, b) => {}
            Instr::I5XY0(x, y) => {}
            Instr::I6XNN(x, b) => self.V[x] = b,
            Instr::I7XNN(x, b) => self.V[x] += b,
            Instr::I8XY0(x, y) => self.V[x] = self.V[y],
            Instr::I8XY1(x, y) => self.V[x] |= self.V[y],
            Instr::I8XY2(x, y) => self.V[x] &= self.V[y],
            Instr::I8XY3(x, y) => self.V[x] ^= self.V[y],
            Instr::I8XY4(x, y) => {
                self.V[F] = arithmetic::check_carry(&self.V[x], &self.V[y]);
                self.V[x] += self.V[y];
            }
            Instr::I8XY5(x, y) => {
                self.V[F] = arithmetic::check_borrow(&self.V[x], &self.V[y]);
                self.V[x] -= self.V[y];
            }
            Instr::I8XY6(x, y) => {
                self.V[F] = arithmetic::get_lsb(&self.V[y]);
                self.V[x] = self.V[y] >> 1;
            }
            Instr::I8XY7(x, y) => {
                self.V[F] = arithmetic::check_borrow(&self.V[x], &self.V[y]);
                self.V[x] = self.V[y] - self.V[x];
            }
            Instr::I8XYE(x, y) => {
                self.V[F] = arithmetic::get_msb(&self.V[y]);
                self.V[x] = self.V[y] << 1;
            }
            Instr::I9XY0(x, y) => {}
            Instr::IANNN(a) => {}
            Instr::IBNNN(a) => self.pc = a + (self.V[0] as u16),
            Instr::ICXNN(x, b) => {
                let r: u8 = rand::thread_rng().gen();
                self.V[x] = r & b;
            }
            Instr::IDXYN(x, y, n) => {}
            Instr::IEX9E(x) => {}
            Instr::IEXA1(x) => {}
            Instr::IFX07(x) => {}
            Instr::IFX0A(x) => {}
            Instr::IFX15(x) => {}
            Instr::IFX18(x) => {}
            Instr::IFX1E(x) => {}
            Instr::IFX29(x) => {}
            Instr::IFX33(x) => {}
            Instr::IFX55(x) => {}
            Instr::IFX65(x) => {}
        }
    }
}
