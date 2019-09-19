use std::num;
use std::str;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum ParsePointError {
    ParseError(num::ParseIntError),
    InsufficientValue,
}

impl str::FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(|p| p.trim())
            .collect();

        if parts.len() != 2 {
            return Err(ParsePointError::InsufficientValue);
        }

        let x;
        let y;

        let parse_x_result = parts[0].parse::<i32>();
        match parse_x_result {
            Err(err) => { return Err(ParsePointError::ParseError(err)); },
            Ok(result) => { x = result },
        }

        let parse_y_result = parts[1].parse::<i32>();
        match parse_y_result {
            Err(err) => { return Err(ParsePointError::ParseError(err)); },
            Ok(result) => { y = result; },
        }

        Ok(Point {
            x: x,
            y: y,
        })
    }
}

pub fn run() {
    let parsed: i32 = "52".parse().unwrap();
    let turbo_parsed = "12".parse::<i32>().unwrap();
    println!("parsed = {}, turbo_parsed = {}", parsed, turbo_parsed);

    let p: Point = "12, 24".parse().unwrap();
    println!("point is: {:?}", p);
}