type Addr = u16;
type Vx = u8;
type Vy = u8;
type Byte = u8;
type Nib = u8;

// lsb = least significant bit

pub enum Instruction {
    SYS(Addr),        // 0NNN
    CLS,              // 00E0
    RET,              // 00EE
    JP(Addr),         // 1NNN
    CALL(Addr),       // 2NNN
    SE(Vx, Byte),     // 3XNN
    SNE(Vx, Byte),    // 4XNN
    SEr(Vx, Vx),      // 5XY0
    LD(Vx, Byte),     // 6XNN
    ADD(Vx, Byte),    // 7XNN
    LDr(Vx, Vy),      // 8XY0
    OR(Vx, Vy),       // 8XY1
    AND(Vx, Vy),      // 8XY2
    XOR(Vx, Vy),      // 8XY3
    ADDr(Vx, Vy),     // 8XY4
    SUBr(Vx, Vy),     // 8XY5
    SHR(Vx, Vy),      // 8XY6
    SUBN(Vx, Vy),     // 8XY7
    SHL(Vx, Vy),      // 8XYE
    SNEr(Vx, Vy),     // 9XY0
    LD(Addr),         // ANNN
    JP0(Addr),        // BNNN
    RND(Vx, Byte),    // CXNN
    DRW(Vx, Vy, Nib), // DXYN
    SKP(Vx),          // EX9E
    SKNP(Vx),         // EXA1
    LDDT(Vx),         // FX07
    LDK(Vx),          // FX0A
    LDseDT(Vx),       // FX15
    LDST(Vx),         // FX18
    ADDI(Vx),         // FX1E
    LDF(Vx),          // FX29
    LDB(Vx),          // FX33
    LDI(Vx),          // FX55
    LDseI(Vx),        // FX65
}
