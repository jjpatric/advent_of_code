use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename: &str = "input.txt";

    let mut max_cals: i32 = 0;
    let mut cals: i32 = 0;
    let mut line_num = 0;
    
    if let Ok(lines) = read_lines(filename) {
        
        for line in lines {
            line_num += 1;
            if let Ok(line) = line {
                match line.parse::<i32>() {
                    Ok(snack_cals) => cals += snack_cals,
                    Err(_) => {
                        if cals > max_cals {
                            max_cals = cals;
                        }
                        cals = 0;
                    }
                }
                println!("\n line_num: {}, cals: {},", line_num, cals);
            }
        }
    }
    println!("\nAnswer: {}", max_cals);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

