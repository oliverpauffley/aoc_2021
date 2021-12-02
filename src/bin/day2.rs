use std::{fmt::Display, panic};

pub struct Position {
    pub x: i32,
    pub y: i32,
    pub aim: i32,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "horizontal position {}\nvertical position {}\nmultiplication {}",
            self.x,
            self.y,
            self.x * self.y
        )
    }
}

pub enum Move {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl From<&str> for Move {
    fn from(a: &str) -> Self {
        let (direction, distance) = a.split_once(" ").unwrap();
        let move_distance = distance.parse::<i32>().unwrap();

        match direction {
            "forward" => Move::Forward(move_distance),
            "down" => Move::Down(move_distance),
            "up" => Move::Up(move_distance),
            _ => panic!("not sure what to do here"),
        }
    }
}

fn main() {
    let input = include_str!("input_day2.txt");

    let pos = parse_input(input);

    println!("{}", pos)
}

fn parse_input(input: &str) -> Position {
    let mut position = Position::new();
    input.lines().for_each(|line| {
        let movement = Move::from(line);
        match movement {
            Move::Forward(distance) => {
                position.x += distance;
                position.y += position.aim * distance;
            }
            Move::Down(distance) => position.aim += distance,
            Move::Up(distance) => position.aim -= distance,
        }
    });
    position
}
