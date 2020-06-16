use hex;
use std::fmt::{Display, Formatter, Result};
use std::fs;
use std::io;

struct ErrInvalidProgram;
impl std::fmt::Display for ErrInvalidProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid program string. Make sure that the program string contains valid instructions in hex form as a string.")
    }
}

pub struct Assembler;
impl Assembler {
    pub fn assemble(hex_prgm: &str, outfile: &str) -> std::io::Result<()> {
        let bytes = hex::decode(hex_prgm).to_owned().unwrap();
        fs::write(outfile, bytes)?;
        Ok(())
    }
}
