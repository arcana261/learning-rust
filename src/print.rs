pub fn run() {
    // prints to stdout
    println!("Hello from print.rs file");

    // Basic formatting
    println!("{} is from {}", "Brad", "Maas");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Named arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

    // Placeholder traits
    println!("Binary: {0:b} Hex: {0:x} Octal: {0:o}", 10);

    // Placeholder for debug trait
    println!("{:?}", (10, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}