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
    let lines_iterator = file_contents.lines();
    let lines: Vec<i32> = lines_iterator
        .map(|line| line.parse::<i32>().expect("Failed to parse line as integer"))
        .collect();
    let mut counter = 0;
    let lines_with_next = lines.iter().zip(lines.iter().skip(1));
    for (current, next) in lines_with_next {
        if current < next {
            counter += 1;
        }
    }

    println!("Ans {}", counter);
}
