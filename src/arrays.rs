use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Slices
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}