use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{Instant};

fn main() {
    let start = Instant::now();

    let mut input_vector = Vec::new();

    // first count up all the first bits
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                input_vector.push(convert_bit_vec(&ip));      
            }
        }
    } 
    let mut in_clone = input_vector.clone();
    
    let oxygen = get_oxygen(input_vector);
    let co2 = get_co2(in_clone);
    let duration = start.elapsed();
    println!("output oxygen: {}, co2 {}, product {}, duration {:?}", oxygen, co2, oxygen*co2, duration);
    
}

fn get_oxygen(vec_ref: Vec<Vec<u32>>) -> u32 {
    let mut input_vector = vec_ref.clone();
    for pos in 0..input_vector[0].len() {
        let mut count_pos = 0;
        for x in &input_vector {
            count_pos += x[pos];
        }
        //println!("count pos {}, vec_len {}", count_pos, input_vector.len());

        if count_pos >= (input_vector.len() as u32) - count_pos {
            input_vector.retain(|x| (*x)[pos] == 1);
        } else {
            input_vector.retain(|x| (*x)[pos] == 0);
        } 
        if input_vector.len() == 1 {
            break;
        }
    }
    
    let mut digits = 0;
    for row in input_vector {
        for i in 0..row.len() {
            if row[i] == 1 {
                digits |= 1;
            } else {
                digits |= 0;
            }
            if i < row.len()-1 {
                digits <<= 1;
            }
        }
    }
    println!("Binary 0b{:b}, oxygen {}", digits, digits);
    return digits;
}

fn get_co2(vec_ref: Vec<Vec<u32>>) -> u32 {
    let mut input_vector = vec_ref.clone();
    for pos in 0..input_vector[0].len() {
        let mut count_pos = 0;
        for x in &input_vector {
            count_pos += x[pos];
        }
        //println!("count pos {}, vec_len {}", count_pos, input_vector.len());

        if count_pos >= (input_vector.len() as u32) - count_pos {
            input_vector.retain(|x| (*x)[pos] == 0);
        } else {
            input_vector.retain(|x| (*x)[pos] == 1);
        }
        if input_vector.len() == 1 {
            break;
        }
    }

    let mut digits = 0;
    for row in input_vector {
        for i in 0..row.len() {
            if row[i] == 1 {
                digits |= 1;
            } else {
                digits |= 0;
            }
            if i < row.len()-1 {
                digits <<= 1;
            }
        }
    }
    println!("Binary 0b{:b}, co2 {}", digits, digits);
    return digits;
}

fn convert_bit_vec(in_str: &str) -> Vec<u32> {
    let mut line_vec = Vec::new();
    for c in in_str.chars() {
        line_vec.push(c.to_digit(10).unwrap_or(0));
    }
    return line_vec;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
