pub fn run() {
    // Infinite Loop
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Number: {}", counter);

        if counter == 20 {
            break;
        }
    }

    // While Loop (FizzBuzz)
    counter = 0;
    while counter <= 100 {
        if counter % 15 == 0 {
            println!("fizzbuzz");
        } else if counter % 3 == 0 {
            println!("fizz");
        } else if counter % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", counter);
        }

        // Inc
        counter += 1;
    }

    // For range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }
}