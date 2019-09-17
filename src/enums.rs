enum Movement {
    Up,
    Right,
    Down,
    Left,
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Avatar moved Up"),
        Movement::Right => println!("Avatar moved Right"),
        Movement::Down => println!("Avatar moved Down"),
        Movement::Left => println!("Avatar moved Left"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}