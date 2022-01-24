
use std::fs;

#[derive(Debug)]
enum Direction {
    FORWARD,
    UP,
    DOWN
}

struct Move {
    direction: Direction,
    number: u16
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_file("/home/alex/development/personal/rust-aoc-2021/d02/input/input.txt");

    let mut horizontal : u32 = 0;
    let mut depth : u32 = 0;

    for mv in input {
        match mv.direction {
            Direction::DOWN => { depth = depth + u32::from(mv.number) },
            Direction::FORWARD => { horizontal = horizontal + u32::from(mv.number) },
            Direction::UP => { depth = depth - u32::from(mv.number) }
        }
    }

    println!("Horizontal : {}, depth {}, multiplied {}", horizontal, depth, horizontal * depth);
}

fn part2() {
    let input = read_file("/home/alex/development/personal/rust-aoc-2021/d02/input/input.txt");

    let mut horizontal : i32 = 0;
    let mut depth : i32 = 0;
    let mut aim : i32 = 0;

    for mv in input {
        match mv.direction {
            Direction::DOWN => { aim = aim + i32::from(mv.number) },
            Direction::FORWARD => { 
                horizontal = horizontal + i32::from(mv.number);
                depth = depth + aim * i32::from(mv.number);
            },
            Direction::UP => { aim = aim - i32::from(mv.number) }
        }
    }

    println!("Horizontal : {}, depth {}, multiplied {}", horizontal, depth, horizontal * depth);
}

fn read_file(filename: &str) -> Vec<Move> {
    let mut data = Vec::new();

    for val in fs::read_to_string(filename).expect("Error reading file.").lines() {
        let split : Vec<&str> = val.split(" ").collect();

        let direction = split[0];
        let number : u16 = split[1].parse().expect("Cannot parse.");

        data.push(Move { direction : parse_direction(direction), number: number });
    }

    data
}

fn parse_direction(direction: &str) -> Direction {
    if direction == "forward" {
        return Direction::FORWARD;
    } else if direction == "up" {
        return Direction::UP;
    } else {
        return Direction::DOWN;
    }
}