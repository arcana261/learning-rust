pub fn run() {
    loop3();
}

fn loop1() {
    let vec = vec![0, 1, 2, 3];

    let mut iter = (&vec).into_iter();
    while let Some(v) = iter.next() {
        println!("{}", v);
    }
}

fn loop2() {
    let vec = vec![0, 1, 2, 3];

    let mut iter = (&vec).into_iter();

    loop {
        match iter.next() {
            Some(v) => println!("{}", v),
            _ => break
        }
    }
}

fn loop3() {
    let vec = vec![0, 1, 2, 3];

    let mut iter = (&vec).into_iter();

    loop {
        let v = match iter.next() {
            Some(v) => v,
            _ => break
        };

        println!("{}", v);
    }
}
