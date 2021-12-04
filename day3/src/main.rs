use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename: &str = "input.txt";
    let ans1: i32 = day3_part1(filename.clone());
    let ans2: i32 = day3_part2(filename.clone());
    println!("Part1 Answer {}", ans1);
    println!("Part2 Answer {}", ans2);
}

fn day3_part1(filename: &str) -> i32{
    // Variables

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(_ip) = line {
                // Do sumthin
            }
        }
    }
    let r: i32 = 1;
    return r;
}

fn day3_part2(filename: &str) -> i32{
    // Variables

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(_ip) = line {
                // Do sumthin
            }
        }
    }
    let r: i32 = 2;
    return r;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

