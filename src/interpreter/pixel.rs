pub enum Pixel {
    ON,
    OFF,
}

impl Pixel {
    fn bit(&self) -> u8 {
        match self {
            ON => 1,
            OFF => 0,
        }
    }
}
