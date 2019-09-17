#![allow(dead_code)]

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff_00_00,
    Green = 0x00_ff_00,
    Blue = 0x00_00_ff,
}

pub fn run() {
    println!("Zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);

    println!("Roses are #{:06X}", Color::Red as i32);
    println!("Violets are #{:06X}", Color::Blue as i32);
}