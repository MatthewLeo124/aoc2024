use std::io::{Result, BufRead, BufReader};
use std::fs::File;
use std::vec::Vec;

fn sol() {
    let input = File::open("src/input.txt").expect("Error reading file");
    let reader = BufReader::new(input);
    let mut sum: i32 = 0;
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                eprintln!("Error parsing line. {}", err);
                continue
            }
        };
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        // Need to compare an element with the previous element here
        let mut increasing = true;
        let mut decreasing = true;
        let mut valid_diff = true;
        for window in nums.windows(2) {
            let diff: i32 = (window[0] - window[1]).abs();
            if diff < 1 || diff > 3 {
                valid_diff = false;
            }

            if window[0] < window[1] {
                decreasing = false;
            } else {
                increasing = false;
            }
        }
        if (increasing || decreasing) && valid_diff {
            sum += 1;
        }
    }
    println!("{}", sum);
    return;
}

fn main() -> Result<()> {
    sol();
    return Ok(())
}
