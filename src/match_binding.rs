pub fn run() {
    match age() {
        0 => println!("I'm not born yet!"),
        n @ 1..=12 => println!("I'm a child of age: {}", n),
        n @ 13..=19 => println!("I'm a teen of age: {}", n),
        n => println!("I'm an old person of age: {}", n),
    }
}

fn age() -> u32 {
    0
}