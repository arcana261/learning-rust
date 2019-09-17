#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

pub fn run() {
    use Status::{Rich, Poor};
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have so much money!"),
        Poor => println!("The poor are out of money :("),
    }

    match work {
        Civilian => println!("Civilans work!"),
        Soldier => println!("Soldiers fight!"),
    }
}