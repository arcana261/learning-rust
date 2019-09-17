use std::thread;

pub fn run() {
    example2();
}

fn example1() {
    let loc = thread::spawn(|| {
        "world"
    });

    println!("Hello, {}!", loc.join().unwrap());
}

fn example2() {
    let v = vec![1, 2, 3];

    thread::spawn(move || {
        println!("v: {:?}", v);
    }).join();
}