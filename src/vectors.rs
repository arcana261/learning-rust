use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(6);
    numbers.push(7);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // Get vector length
    println!("Vector length: {}", numbers.len());

    // Vector memory
    println!("Vector occupies {} bytes in RAM", mem::size_of_val(&numbers));

    // Slices
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector items
    for x in numbers.iter() {
        println!("Item: {}", x);
    }

    // Loop and mutate each value
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("After mutate: {:?}", numbers);
}