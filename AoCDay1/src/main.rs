use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{Instant};

fn main() {
    let start = Instant::now();

    let mut prev_value1: u32 = 0;
    let mut prev_value2: u32 = 0;
    let mut prev_value3: u32 = 0;
    let mut inc_counter = 0;
    let mut i = 0;

    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                i += 1;
                let current_value: u32 = ip.parse().unwrap();
                if i > 3 {
                    inc_counter += compare3_counter(prev_value1, prev_value2, prev_value3, current_value);
                }
                prev_value3 = prev_value2;
                prev_value2 = prev_value1;
                prev_value1 = current_value;
            }
        }
    }
    let duration = start.elapsed();

    println!("inc counter: {}, time: {:?}", inc_counter, duration);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn compare3_counter(prev_value1: u32, prev_value2: u32, prev_value3: u32, current_value: u32) -> u32 {
    let prev3 = prev_value1 + prev_value2 + prev_value3;
    let cur3 = prev_value1 + prev_value2 + current_value;
    //println!("p1: {}, p2: {}, p3 {}, cur {}, p123: {}, pC12 {}", prev_value1, prev_value2, prev_value3, current_value, prev3, cur3);
    if cur3 > prev3 {
        return 1;
    }
    else {
        return 0;
    }
}