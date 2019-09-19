#[derive(Debug)]
struct Number {
    value: i32
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number {
            value: item,
        }
    }
}

impl Into<i32> for Number {
    fn into(self) -> i32 {
        self.value
    }
}

pub fn run() {
    let x = 5i32;
    let y: Number = Number::from(x);

    println!("{:?}", y);

    let z: i32 = y.into();
    println!("{}", z);
}