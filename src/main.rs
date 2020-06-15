use chip8::arithmetic;
use chip8::interpreter;

fn main() {
    let c8 = interpreter::chip8::Chip8::new();
    c8.memory_dump();
}
