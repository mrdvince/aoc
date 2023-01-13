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
    println!("Ans {}", counter);
}
