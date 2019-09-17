#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

#[derive(Debug)]
#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Rectangle {
            p1: Point{
                x: x1,
                y: y1,
            },
            p2: Point{
                x: x2,
                y: y2,
            },
        } = self;

        ((x2 - x1) * (y2 - y1)).abs()
    }
}

fn square(p: &Point, width: f32) -> Rectangle {
    let Point{x: point_x, y: point_y} = p;

    Rectangle {
        p1: p.clone(),
        p2: Point{x: point_x + width, y: point_y + width},
    }
}

pub fn run() {
    let name = "mehdi";
    let age = 30;
    let mehdi = Person{name: name, age: age};

    println!("{:?}", mehdi);

    let point = Point{x: 0.3, y: 0.4};
    println!("Point coordinates: ({}, {})", point.x, point.y);

    let new_point = Point{x: 0.1, ..point};
    println!("second point coordinates: ({}, {})", new_point.x, new_point.y);

    let Point{x: my_x, y: my_y} = point;
    println!("my_x={}, my_y={}", my_x, my_y);

    let _rectangle = Rectangle{
        p1: Point{x: my_x, y: my_y},
        p2: point.clone(),
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);
    println!("Pair contains: {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("Pair contains: {} and {}", integer, decimal);

    let sample_rectangle = Rectangle{
        p1: Point{x: 1.0, y: 2.0},
        p2: Point{x: 3.0, y: 5.0},
    };

    println!("area: {}", sample_rectangle.rect_area());

    println!("sqaure: {:?}", square(&point, 3.0));
}