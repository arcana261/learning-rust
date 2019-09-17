pub fn run() {
    println!("Foo");
    println!("5 => {:?}", foo(Some(5)));
    println!("0 => {:?}", foo(Some(0)));
    println!("-1 => {:?}", foo(Some(-1)));
    println!("None => {:?}", foo(None));
    println!("Bar");
    println!("5 => {:?}", bar(Some(5)));
    println!("0 => {:?}", bar(Some(0)));
    println!("-1 => {:?}", bar(Some(-1)));
    println!("None => {:?}", bar(None));
}

fn foo1(input: Option<i32>) -> Option<i32> {
    match input {
        Some(i) if i >= 0 => Some(i),
        Some(_x) => None,
        None => None
    }
}

fn foo2(input: Option<i32>) -> Option<i32> {
    let value = input?;

    if value < 0 {
        return None
    }
    Some(value)
}

fn foo3(input: Option<i32>) -> Option<i32> {
    input.and_then(|x| {
        if x < 0 {
            None
        } else {
            Some(x)
        }
    })
}

fn foo(input: Option<i32>) -> Option<i32> {
    input.filter(|i| i >= &0)
}

fn bar(input: Option<i32>) -> Result<i32, String> {
    foo(input).ok_or(String::from("negative"))
}
