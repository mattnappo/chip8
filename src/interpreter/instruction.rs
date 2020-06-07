type Addr = u16;
type Vx = usize;
type Vy = usize;
type Byte = u8;
type Nib = u8;

// lsb = least significant bit

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
    ICNNN(Vx, Byte),    // CXNN
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
}
