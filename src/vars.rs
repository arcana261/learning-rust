pub fn run() {
    let name = "Mehdi";
    let mut age = 30;

    println!("My name is {} and I am {} years old", name, age);

    age = 31;

    println!("My name is {} and I am {} years old", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Multiple vars
    let (my_name, my_age) = ("Mehdi", 30);
    println!("{} is {}", my_name, my_age);
}