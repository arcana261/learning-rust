#![allow(unreachable_code)]

pub fn run() {
    'outer: loop {
        println!("entered the outer loop!");

        'inner: loop {
            println!("entered the inner loop!");

            break 'outer;
        }

        println!("unreachable code!");
    }

    println!("exited the outer loop!");
}