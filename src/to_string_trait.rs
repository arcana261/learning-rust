use std::fmt;

struct Circle {
    radius: f32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle(r={})", self.radius)
    }
}

pub fn run() {
    let c = Circle { radius: 5.2 };
    let s: String = c.to_string();
    println!("s = {}", s);
}