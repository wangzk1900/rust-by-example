// Rust 不提供原始类型的隐式转换

#![allow(overflowing_literals)]
fn main() {
    let decimal = 65.4321_f32;

    // let integer: u8 = decimal; // mismatched types

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // let character = decimal as char; // only `u8` can be cast as `char`, not `f32`

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is: {}", 1000 as u8); // 1000-256-256-256 = 232
    println!("-1 as a u8 is: {}", (-1i8) as u8); // -1 + 256 = 255

    println!("1000 mod 256 is: {}", 1000 % 256);

    println!(" 128 as a i16 is: {}", 128 as i16);
    println!(" 128 as a i8 is: {}", 128 as i8);

    println!("1000 as a u8 is: {}", 1000 as u8);
    println!("232 as a u8 is: {}", 232 as i8);
}