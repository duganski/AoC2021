use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{Instant};

#[derive(Debug)]
struct Direction {
    forward: i32,
    down: i32,
    aim: i32
}

fn main() {
    let start = Instant::now();

    let mut dir = Direction {forward: 0, down: 0, aim: 0};

    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let vec_lines: Vec<&str> = ip.split(' ').collect();
                //println!("vec[0]: {} vec[1]: {}", vec_lines[0], vec_lines[1]);
                let new_dir = direction(vec_lines[0], vec_lines[1].parse().unwrap(), dir.aim);
                //println!("new_dir: {:?}", new_dir);
                dir.forward += new_dir.forward;
                dir.aim += new_dir.aim;
                dir.down += new_dir.down;
                //println!("dir: {:?}", dir);
            }
        }
    }
    let duration = start.elapsed();

    println!("distance {}, time: {:?}", dir.forward*dir.down, duration);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn direction(dir: &str, dist: i32, curr_aim: i32) -> Direction {
    let mut new_dir = Direction {forward: 0, down: 0, aim: 0};
    match dir {
        "forward" => {
            new_dir.forward = dist;
            new_dir.down = curr_aim*dist;
        }
        "up" => new_dir.aim = -dist,
        "down" => new_dir.aim = dist,
        _ => println!("error with dir"),
    }   
    return new_dir;
}