pub fn run() {
    iter_example();
    into_iter_example();
    iter_mut_example();
}

// iter borrows each element so the whole
// vector can be used outside for loop
fn iter_example() {
    println!("BEGIN iter_example");

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustocean among us!"),
            _ => println!("Hello, {}", name),
        }
    }

    // since each element is borrowed
    // we can do this

    println!("vector: {:?}", names);

    println!("END iter_example");
}

// into_iter moves vector inside for loop so it
// is illegal to use it after loop. also each element
// is therefore moved
//
// also the default behaviour of for loop uses into_iter
fn into_iter_example() {
    println!("BEGIN into_iter_example");

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustocean among us!"),
            _ => println!("Hello, {}", name),
        }
    }

    println!("END into_iter_example");
}

// iter_mut borrows each element allowing vector to be used
// outside of for. also each element is a mutable reference
// so vector can be changed in place.
fn iter_mut_example() {
    println!("BEGIN iter_mut_example");

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustocean among us!"),
            _ => println!("Hello, {}", name),
        }
    }

    println!("contents of vector: {:?}", names);

    println!("END iter_mut_example");
}