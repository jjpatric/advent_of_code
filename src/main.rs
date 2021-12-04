use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename: &str = "input.txt";

    let mut h: i32 = 0;
    let mut d: i32 = 0;
    let mut a: i32 = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let i: Vec<&str> = ip.split(' ').collect();
                let c: &str = i[0];
                let x: i32 = i[1].parse::<i32>().expect("Invalid input");
                match c {
                    "forward" => {
                        h += x;
                        d += a*x;
                    },
                    "down" => {
                        a += x;
                    },
                    "up" => {
                        a -= x;
                    },
                    _ => {
                        println!("Invalid input {}", ip);
                    },
                }
            }
        }
    }
    let r: i32 = h*d;
    println!("h: {}", h);
    println!("d: {}", d);
    println!("\nAnswer: {}", r);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

