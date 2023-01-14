use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file path as an argument.");
        return;
    }
    let file_path = &args[1];
    let file_contents = fs::read_to_string(file_path).expect("Failed to open data file");

    let part_one_ans = part_one(&file_contents);
    let part_two_ans = part_two(&file_contents);

    println!("Ans part one {}", part_one_ans);
    println!("Ans part two {}", part_two_ans);
}
fn part_two(file_contents: &String) -> i32 {
    let lines: Vec<_> = file_contents
        .lines()
        .skip(1)
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();
    let mut count = 0;
    let mut prev_sum = 0;
    for window in lines.windows(3) {
        let sum = window.iter().sum();
        if sum > prev_sum {
            count += 1;
        }
        prev_sum = sum;
    }
    count
}

fn part_one(file_contents: &String) -> i32 {
    let counter = file_contents
        .lines()
        .skip(1)
        .filter_map(|line| line.parse::<i32>().ok())
        .fold((0, 0), |(count, prev), curr| {
            if prev < curr {
                (count + 1, curr)
            } else {
                (count, curr)
            }
        })
        .0;
    counter
}
