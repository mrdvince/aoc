use std::{env, fs::File, io::Read};

fn main() {
    let file_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Please provide a file path as an argument.");
            return;
        }
    };
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Failed to read file");
    let (horizontal, depth, res) = part_one(&file_contents);
    println!("Part One answer: {}, {}, {}", horizontal, depth, res);

    let (horizontal, depth, res) = part_two(&file_contents);
    println!("Part Two answer: {}, {}, {}", horizontal, depth, res)
}
fn part_two(file_contents: &String) -> (i32, i32, i32) {
    let (mut horizontal, mut depth, mut aim) = (0, 0, 0);
    for line in file_contents.lines() {
        let direction_depth: Vec<&str> = line.split(" ").filter(|s| !s.is_empty()).collect();
        if direction_depth.len() < 2 {
            continue;
        }
        match direction_depth[0] {
            "forward" => {
                let forward_value = direction_depth[1].parse().unwrap_or(0);
                horizontal += forward_value;
                depth += aim * forward_value;
            }
            "down" => aim += direction_depth[1].parse().unwrap_or(0),
            "up" => aim -= direction_depth[1].parse().unwrap_or(0),
            _ => continue,
        }
    }
    let res = horizontal * depth;
    (horizontal, depth, res)
}

fn part_one(file_contents: &String) -> (i32, i32, i32) {
    let (mut horizontal, mut depth) = (0, 0);
    for line in file_contents.lines() {
        let direction_depth: Vec<&str> = line.split(" ").filter(|s| !s.is_empty()).collect();
        if direction_depth.len() < 2 {
            continue;
        }
        match direction_depth[0] {
            "forward" => horizontal += direction_depth[1].parse().unwrap_or(0),
            "down" => depth += direction_depth[1].parse().unwrap_or(0),
            "up" => depth -= direction_depth[1].parse().unwrap_or(0),
            _ => continue,
        }
    }
    let res = horizontal * depth;
    (horizontal, depth, res)
}
