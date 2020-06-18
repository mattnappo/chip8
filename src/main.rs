use chip8::arithmetic;
use chip8::assembler;
use chip8::interpreter;

fn generate_test_program() {
    let program = "62ff63ab8520";
    assembler::Assembler::assemble(program, "roms/TEST.bin")
        .expect("Could not assemble program");
}

fn main() {
    // generate_test_program();
    let mut c8 = interpreter::chip8::Chip8::new();
    println!("\n\n");

    c8.load_rom("roms/PONG.bin").expect("Could not load ROM");
    // c8.memory_dump(250);
    c8.full_dump();

    c8.run();

    for i in 0x200..0x210 {
        println!("memory @ 0x{:x}: 0x{:x}", i, c8.memory[i]);
    }
}
