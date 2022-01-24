use std::fs;

fn main() {
    part1();
    part2();
}

fn read_file(filename: &str) -> Vec<u32> {
    let mut data = Vec::new();

    for val in fs::read_to_string(filename).expect("Error reading file.").lines() {
        data.push(val.trim().parse().expect("Cannot parse to number."));
    }

    data
}

fn part1() {
    let data = read_file("/home/alex/development/personal/rust-aoc-2021/d01/input/part1.txt");
    let mut num_larger : u16 = 0;
    let mut current : u32 = u32::MAX;

    for point in data {
        if point > current {
            num_larger = num_larger + 1;
        }

        current = point;
    }

    println!("Result : {}", num_larger);
}

fn part2() {
    let data = read_file("/home/alex/development/personal/rust-aoc-2021/d01/input/part1.txt");
    let mut window : [u32 ; 3] = [0, 0, 0];
    let mut window_size : u8 = 0;
    let mut current_sum : u32 = 0;
    let mut num_larger : u32 = 0;

    for point in data {
        window[2] = window[1];
        window[1] = window[0];
        window[0] = point;

        if window_size < 3 {
            window_size = window_size + 1
        } 

        if window_size == 3 {
            let sum = window[0] + window[1] + window[2];

            if sum > current_sum && current_sum != 0{
                num_larger = num_larger + 1
            }

            current_sum = sum;
        }
    }

    println!("Result : {}", num_larger);
}