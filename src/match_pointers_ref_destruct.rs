pub fn run() {
    let reference = &4;

    match reference {
        &val => println!("Got value by destructing reference: {:?}", val),
    }

    match *reference {
        val => println!("Got value by dereferencing first: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 4;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref val => println!("Acquired a reference to a value: {:?}", val),
    }

    match mut_value {
        ref mut val => {
            *val += 10;
            println!("final value of mut_value: {:?}", val);
        },
    }
}