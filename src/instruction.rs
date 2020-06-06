type Address = u16;
type Register = u8;
type Byte = u8;

pub enum Instruction {
    SYS(Address),            // Jump to routine
    CLS,                     // Clear the screen
    RET,                     // Return from a subroutine
    JP(Address),             // Jump to location
    SE(Register, Byte),      // Skip next instruction if Vx = kk
    SNE(Register, Byte),     // Skip next instruction if Vx != kk
    SEr(Register, Register), // Skip next instruction if Vx != Vy
}
