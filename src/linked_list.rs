enum List<T> {
    Value(T, Box<List<T>>),
    Nil,
}

impl<T: std::fmt::Display> List<T> {
    fn new() -> List<T> {
        List::Nil
    }

    fn prepend(self, elem: T) -> List<T> {
        List::Value(elem, Box::new(self))
    }

    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Value(_, next) => 1 + next.len(),
        }
    }

    fn stringify(&self) -> String {
        match self {
            List::Nil => format!("Nil"),
            List::Value(head, tail) => format!("{} {}", head, tail.stringify()),
        }
    }
}

pub fn run() {
    let mut list: List<i32> = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("size = {}", list.len());
    println!("list is: {}", list.stringify());
    println!("size = {}", list.len());
    println!("list is: {}", list.stringify());
}