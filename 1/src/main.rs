use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::vec::Vec;
use std::collections::HashMap;

fn sol() {
    let file = File::open("src/input.txt").expect("Error reading file");
    let reader = BufReader::new(file);    
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    let mut freq2: HashMap<i32, i32> = HashMap::new();

    for l in reader.lines() {
        let line = match l {
            Ok(line) => line,
            Err(err) => {
                eprintln!("Error parsing line. {}", err);
                continue;
            }
        };
        let items: Vec<&str> = line.split_whitespace().collect();

        if items.len() < 2 {
            eprintln!("Less than 2 items on line");
            continue;
        }
        
        if let (Ok(l1), Ok(l2)) = (items[0].parse::<i32>(), items[1].parse::<i32>()) {
            v1.push(l1);
            v2.push(l2);
            *freq2.entry(l2).or_insert(0) += 1;
        } else {
            eprintln!("Data error");
        }
    };
    v1.sort();
    v2.sort();
    let sum: i32 = v1
        .iter()
        .zip(v2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
    println!("part 1: {}", sum);
    
    let sum2: i32 = v1
        .iter()
        .filter_map(|&x| freq2.get(&x).map(|&count| x * count))
        .sum();
    println!("part 2: {}", sum2);
}

fn main() -> Result<()> {
    sol();
    Ok(())
}
