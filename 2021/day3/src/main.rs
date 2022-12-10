use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use array_tool::vec::Intersect;

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
    
    let r: i32 = gamma_rate * epsilon_rate;
    return r;
}

fn day3_part2(filename: &str) -> i32{
    // Variables
    let mut bit_balance: Vec<i32> = Vec::new();
    let mut all_entries: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                // Do sumthin
                all_entries.push(ip.clone());
                if bit_balance.len() == 0 { bit_balance = vec![0; ip.len()]; }
                for (i, c) in ip.chars().enumerate() {
                    let bit: u32 = c.to_digit(2).unwrap();
                    match bit {
                        0 => bit_balance[i] -= 1,
                        1 => bit_balance[i] += 1,
                        _ => println!("invalid bit"),
                    }
                }
            }
        }
    }

    let mut co2: i32 = 0;
    let mut oxy: i32 = 0;
    let mut co2_filter_bit: i32 = 0;
    let mut oxy_filter_bit: i32 = 0;
    let mut co2_set:Vec<String> = all_entries.clone();
    let mut oxy_set:Vec<String> = all_entries.clone();
    let mut co2_bit_balance: Vec<i32> = bit_balance.clone();
    let mut oxy_bit_balance: Vec<i32> = bit_balance.clone();

    println!("bit balance: {:?}", bit_balance);
    for i in 0..bit_balance.len() {
        let mut co2_matching_entries:Vec<String> = Vec::new();
        let mut oxy_matching_entries:Vec<String> = Vec::new();
        let mut co2_bit: i32;
        if i < co2_bit_balance.len() { co2_bit = co2_bit_balance[i]; }
        else { co2_bit = 0 }
        let mut oxy_bit: i32;
        if i < oxy_bit_balance.len() { oxy_bit = oxy_bit_balance[i]; }
        else { oxy_bit = 0 }

        if co2_bit < 0 { co2_filter_bit = 1 }
        else { co2_filter_bit = 0 }

        if oxy_bit < 0 { oxy_filter_bit = 0 }
        else { oxy_filter_bit = 1 }
        println!("filter bits co2: {:?} oxy: {:?}", co2_filter_bit, oxy_filter_bit);


        for entry in all_entries.iter() {
            if entry.chars().nth(i).unwrap().to_digit(2).unwrap() == co2_filter_bit.try_into().unwrap() {
                co2_matching_entries.push(entry.to_string());
            }
            if entry.chars().nth(i).unwrap().to_digit(2).unwrap() == oxy_filter_bit.try_into().unwrap() {
                oxy_matching_entries.push(entry.to_string());
            }
        }
        co2_set = co2_set.intersect(co2_matching_entries.clone());
        co2_bit_balance = get_bit_balance(co2_set.clone());
        oxy_set = oxy_set.intersect(oxy_matching_entries.clone());
        oxy_bit_balance = get_bit_balance(oxy_set.clone());

        println!("Matching Sets co2: {:?} oxy: {:?}", co2_set, oxy_set);
        if co2_set.len() == 1 {
            let winning_entry = co2_set.pop().unwrap();
            println!("co2 winning entry: {}", winning_entry);
            co2 = string_to_binary(winning_entry.clone());
        }
        if oxy_set.len() == 1 {
            let winning_entry = oxy_set.pop().unwrap();
            println!("oxy winning entry: {}", winning_entry);
            oxy = string_to_binary(winning_entry.clone());
        }
    }  
    
    let r: i32 = co2 * oxy;
    return r;
}

fn get_bit_balance(v: Vec<String>) -> Vec<i32> {
    let mut bit_balance: Vec<i32> = Vec::new();
    for s in v {
        for (i, c) in s.chars().enumerate() {
            if bit_balance.len() == 0 { bit_balance = vec![0; s.len()]; }
            let bit: u32 = c.to_digit(2).unwrap();
            match bit {
                0 => bit_balance[i] -= 1,
                1 => bit_balance[i] += 1,
                _ => println!("invalid bit"),
            }
        }
    }
    bit_balance
}

fn string_to_binary(s: String) -> i32 {
    let mut value = 0;
    for bit in s.chars() {
        value *= 2;
        if bit == '1' {
            value += 1;
        }
    } 
    value
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

