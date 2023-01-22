use std::{env, fs::File, io::Read};

fn main() {
    let file_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Please provide a file path as an argument.");
            return;
        }
    };
    let file_contents = read_file(&file_path);
    let res = part_one(&file_contents);
    println!("Part One: {}", res)
}
fn part_one(file_contents: &String) -> i32 {
    let diagnostic_report: Vec<String> =
        file_contents.lines().map(|line| line.to_string()).collect();
    // println!("{:?}", diagnostic_report);
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    // Iterate through each bit position
    for i in 0..diagnostic_report[0].len() {
        let mut ones = 0;
        let mut zeroes = 0;
        for report in &diagnostic_report {
            if report.chars().nth(i).unwrap() == '0' {
                zeroes += 1;
            } else {
                ones += 1;
            }
        }
        // Determine the most common and least common bit
        gamma_rate.push(if ones > zeroes { '1' } else { '0' });
        epsilon_rate.push(if ones < zeroes { '1' } else { '0' });
    }
    let gamma_rate: i32 = i32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate: i32 = i32::from_str_radix(&epsilon_rate, 2).unwrap();
    let power_consumption = gamma_rate * epsilon_rate;

    power_consumption
}
fn read_file(file_path: &String) -> String {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Failed to read file");
    file_contents
}
