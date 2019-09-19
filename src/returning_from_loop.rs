pub fn run() {
    let x = {
        let mut counter = 0;
        loop {
            counter = counter + 1;

            if counter == 10 {
                break counter * 2;
            }
        }
    };

    println!("x = {}", x);
    assert_eq!(x, 20);
}