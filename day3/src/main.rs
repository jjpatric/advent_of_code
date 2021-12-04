use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename: &str = "input.txt";
    let ans1: i32 = day3_part1(filename);
    let ans2: i32 = day3_part2(filename);
    println!("Part1 Answer {}", ans1);
    println!("Part2 Answer {}", ans2);
}

fn day3_part1(filename: &str) -> i32{
    // Variables
    let mut bit_balance: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                // Do sumthin
                if bit_balance.len() == 0 { bit_balance = vec![0; ip.len()]; }
                for (i, c) in ip.chars().enumerate() {
                    let bit: u32 = c.to_digit(2).unwrap();
                    match bit {
                        0 => bit_balance[i] += 1,
                        1 => bit_balance[i] -= 1,
                        _ => println!("invalid bit"),
                    }
                }
            }
        }
    }

    let mut gamma_rate: i32 = 0;
    let mut epsilon_rate: i32 = 0;
    for bit in bit_balance {
        gamma_rate *= 2;
        epsilon_rate *= 2;
        if bit < 0 {
            gamma_rate += 1;
        } 
        else if bit >= 0 {
            epsilon_rate += 1;
        }
    }
    println!("{}", gamma_rate);
    println!("{}", epsilon_rate);    
    
    let r: i32 = gamma_rate * epsilon_rate;
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

