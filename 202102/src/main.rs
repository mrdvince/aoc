use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file path as an argument.");
        return;
    }
    let mut file = File::open(&args[1]).expect("Failed to open file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Failed to read file");

    let mut horizontal = 0;
    let mut depth = 0;

    for line in file_contents.lines() {
        let split = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let direction = split[0];
        let distance = split[1];
        if direction =="forward" {
            horizontal += distance.parse().unwrap_or(0);
        }
        if direction =="down" {
            depth += distance.parse().unwrap_or(0);
        }
        if direction =="up" {
            depth -= distance.parse().unwrap_or(0);
        }
    }
    let res = horizontal * depth;
    println!("{}, {}, {}", horizontal, depth, res)
}
