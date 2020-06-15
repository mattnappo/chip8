use chip8::arithmetic;
use chip8::interpreter;

fn main() {
    let mut c8 = interpreter::chip8::Chip8::new();
    c8.memory_dump(100);

    c8.load_rom("roms/PONG.bin").expect("Could not load ROM");
    c8.memory_dump(250);
}
