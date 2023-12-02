use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(data) = read_lines("input.txt") {
        let mut result: Vec<String> = Vec::new();
        for item in data {
            let mut first = String::new();
            let mut last = String::new();
            if let Ok(item) = item {
                for c in item.chars() {
                    if c.is_ascii_digit() {
                        if first.is_empty() {
                            first = c.to_string();
                        }
                        last = c.to_string();
                    }
                }
            }
            result.push(first + &last);
        }
        let end: i32 = result.iter().map(|x| x.parse::<i32>().unwrap()).sum();
        println!("{:?}", end);
    }
}
