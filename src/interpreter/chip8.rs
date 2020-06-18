use super::super::gfx;
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
pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
const F: usize = 0xF;

const FONTSET_SIZE: usize = 80;
const FONT_HEIGHT: usize = 8;
const FONT_WIDTH: usize = 5;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10,
    0xF0, 0x80, 0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10,
    0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0,
    0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80,
    0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

const N_KEYS: usize = 16;
const KEYS: [u8; N_KEYS] = [
    0x1, 0x2, 0x3, 0xC, 0x4, 0x5, 0x6, 0xD, 0x9, 0x8, 0x9, 0xE, 0xA, 0x0, 0xB,
    0xF,
];

pub const OFF: u8 = 0x0;
pub const ON: u8 = 0x1;

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

    display: gfx::Display,
    keys: [u8; N_KEYS], // The keys
    key: Option<u8>,    // The current key being pressed
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

            display: gfx::Display::new(),
            keys: [0; N_KEYS], // Not actually initialized like this
            key: None,
        };
        c8.install_fontset();
        c8.init_keys();
        c8
    }

    // run runs the contents of the virtual machine.
    pub fn run(&mut self) {
        // Run the window
        while let Some(event) = self.display.screen.next() {
            // Run the program until it stops
            while self.pc < self.memory.len() as u16 {
                // Step the processor once
                self.cycle();
                // Draw the screen
                self.display.draw(&event);
            }
        }
    }

    // install_fontset loads the font ROM into memory.
    fn install_fontset(&mut self) {
        for i in 0..FONTSET.len() {
            println!("fontset[{}] = {:x}", i, FONTSET[i]);
            self.memory[i] = FONTSET[i];
        }
    }

    // init_keys initializes the keypad.
    fn init_keys(&mut self) {
        for i in 0..KEYS.len() {
            self.keys[i] = KEYS[i];
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

    // cycle will step the virtual machine once.
    pub fn cycle(&mut self) {
        // Count down the timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        // println!("program counter: {}", self.pc);

        // Fetch an instruction
        // The only reason why these are u16s is because it will make them easier
        // to deal with when determining the instruction.
        let current_instr: u16 = self.memory[self.pc as usize].into();
        let next_instr: u16 = self.memory[self.pc as usize + 1].into();
        let opcode: u16 = (current_instr << 8) | next_instr;
        let jmp: u16 = (opcode & 0x0FF).into(); // NNN (the jump address)
        let x: usize = ((opcode & 0x0F00) >> 8).into();
        let y: usize = ((opcode & 0x00F0) >> 4).into();
        let nn: u16 = opcode & 0x00FF;

        // Execute the fetched instruction
        let instr = Instruction::get_instr_from_parts(opcode, jmp, x, y, nn);
        self.execute(instr);
    }

    // execute executes a single instruction.
    fn execute(
        &mut self,
        i: Instruction,
    ) -> Result<(), ErrUnsupportedInstruction> {
        let mut should_jump = false;
        match i {
            Instruction::I0NNN(a) => {} // Not really implemented
            Instruction::I00E0 => {
                // Clear the display
                for i in 0..self.display.pixels.len() {
                    self.display.pixels[i] = OFF;
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
                self.stack[self.sp as usize] = self.pc as u8; // Push current address to the stack
                self.pc = a;
                should_jump = true;
            }
            Instruction::I3XNN(x, b) => {
                if self.V[x] == b {
                    self.pc += 2;
                }
            }
            Instruction::I4XNN(x, b) => {
                if self.V[x] != b {
                    self.pc += 2;
                }
            }
            Instruction::I5XY0(x, y) => {
                if self.V[x] == self.V[y] {
                    self.pc += 2;
                }
            }
            Instruction::I6XNN(x, b) => {
                self.V[x] = b;
                println!("V[{}] = {}", x, b)
            }
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
            Instruction::I9XY0(x, y) => {
                if self.V[x] != self.V[y] {
                    self.pc += 2;
                }
            }
            Instruction::IANNN(a) => self.I = a,
            Instruction::IBNNN(a) => self.pc = a + (self.V[0] as u16),
            Instruction::ICXNN(x, b) => {
                let r: u8 = rand::thread_rng().gen();
                self.V[x] = r & b;
            }
            Instruction::IDXYN(x, y, n) => {
                // Draw sprite at position (Vx, Vy) with N bytes of sprite data starting
                // at the address stored in I. Set VF to 01 if any set pixels are
                // changed to unset, and 00 otherwise.
                self.V[F] = 1;
                let xpos = self.V[x];
                let ypos = self.V[y];
                let mut sprite: Vec<u8> = Vec::new();
                for i in 0..n {
                    sprite.push(self.memory[self.I as usize + i as usize]);
                }

                for i in 0..FONT_HEIGHT {
                    for j in 0..FONT_WIDTH {}
                }

                /*
                println!("FONT: ");
                for byte in sprite.clone() {
                    println!("{:#018b}", byte);
                }
                */

                let mut yi = ypos as usize;
                for byte in sprite {
                    for bit in 0..8 {
                        let mut xi = (xpos + (0x7 - bit)) as usize;
                        if xi > 0x3F {
                            xi = 0x45;
                        }
                        if yi > 0x1F {
                            yi -= 0x20;
                        }

                        let mut flipped = OFF;
                        if (byte >> bit) & 1 == 1 {
                            flipped = ON;
                        }

                        if flipped != self.display.pixels[WIDTH * yi + xi] {
                            self.display.pixels[WIDTH * yi + xi] = ON;
                        } else {
                            if self.display.pixels[WIDTH * yi + xi] == ON {
                                self.V[F] = 1;
                            }
                            self.display.pixels[WIDTH * yi + xi] = OFF;
                        }
                    }
                    yi += 1;
                }
            }
            Instruction::IEX9E(x) => match self.key {
                Some(k) => {
                    if self.V[x] == k {
                        self.pc += 2;
                    }
                }
                None => (),
            },
            Instruction::IEXA1(x) => match self.key {
                Some(k) => {
                    if self.V[x] != k {
                        self.pc += 2;
                    }
                }
                None => (),
            },
            Instruction::IFX07(x) => self.V[x] = self.delay_timer as u8,
            Instruction::IFX0A(x) => loop {
                match self.key {
                    Some(k) => self.V[x] = k,
                    None => continue,
                }
            },
            Instruction::IFX15(x) => self.delay_timer = self.V[x] as u16,
            Instruction::IFX18(x) => self.sound_timer = self.V[x] as u16,
            Instruction::IFX1E(x) => self.I += self.V[x] as u16,
            Instruction::IFX29(x) => {
                let sprite: usize = (5 * (self.V[x] + 1)).into();
                self.I = self.memory[sprite] as u16;
            }
            Instruction::IFX33(x) => {
                self.memory[self.I as usize] = self.V[x] / 100;
                self.memory[self.I as usize + 1] = (self.V[x] / 10) % 10;
                self.memory[self.I as usize + 2] = self.V[x] % 10;
            }
            Instruction::IFX55(x) => {
                for i in 0..(x + 1) {
                    self.memory[self.I as usize + i] = self.V[i];
                }
                self.I += (x + 1) as u16;
            }
            Instruction::IFX65(x) => {
                for i in 0..(x + 1) {
                    self.V[i] = self.memory[self.I as usize + i];
                }
                self.I += (x + 1) as u16;
            }

            Instruction::UNSUPPORTED => return Err(ErrUnsupportedInstruction),
        };

        if !should_jump {
            self.pc += 2;
        }
        Ok(())
    }
}
