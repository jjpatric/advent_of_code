use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename: &str = "input.txt";

    let mut top_3: Vec<i32> = vec![0, 0, 0];
    let mut cals: i32 = 0;
    let mut line_num = 0;
    
    if let Ok(lines) = read_lines(filename) {
        
        for line in lines {
            line_num += 1;
            if let Ok(line) = line {
                match line.parse::<i32>() {
                    Ok(snack_cals) => cals += snack_cals,
                    Err(_) => {
                        let min_top_3 = *top_3.iter().min().unwrap();
                        if cals > min_top_3 {
                            let index_of_min: usize = get_min_val_index(&top_3);
                            top_3[index_of_min] = cals;
                        }
                        cals = 0;
                    }
                }
                println!("\n line_num: {}, cals: {},", line_num, cals);
            }
        }
    }
    println!("\nAnswer: {}", top_3.iter().sum::<i32>());
}

fn get_min_val_index(nets: &Vec<i32>) -> usize{
    let index_of_min: Option<usize> = nets
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index);
    return index_of_min.unwrap();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

