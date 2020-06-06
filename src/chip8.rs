use crate::instruction;

const MEM_SIZE: usize = 0x1000;
const N_REGISTERS: usize = 16;
const STACK_DEPTH: usize = 12;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Chip8 {
    memory: [u8; MEM_SIZE],   // The memory
    V: [u8; N_REGISTERS],     // The general purpose registers
    I: u16,                   // The I register
    stack: [u8; STACK_DEPTH], // The stack

    pc: u16, // The program counter
    sp: u8,  // The stack pointer

    delay_timer: u16, // The delay timer
    sound_timer: u16, // The sound timer
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: [0; MEM_SIZE],
            V: [0; N_REGISTERS],
            I: 0,
            stack: [0; STACK_DEPTH],
            pc: 0,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn execute_instruction(&mut self, i: instruction::Instruction) {}
}
