pub fn check_carry(x: &u8, y: &u8) -> u8 {
    match x.checked_add(*y) {
        Some(_) => 0, // If there is not a carry
        None => 1,    // If there is a carry
    }
}

pub fn check_borrow(x: &u8, y: &u8) -> u8 {
    // If no borrow
    if x > y {
        1
    }
    // If borrow (if y > x)
    0
}
