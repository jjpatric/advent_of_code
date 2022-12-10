use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename: &str = "input.txt";
    let ans1: i32 = part1(filename);
    let ans2: i32 = part2(filename);
    println!("Part1 Answer {}", ans1);
    println!("Part2 Answer {}", ans2);
}

fn part1(filename: &str) -> i32{
    // Variables
    const BOARD_N: usize = 5;
    let mut num_seq: Vec<i32> = Vec::new();
    let mut game_board = [0; BOARD_N*BOARD_N];
    let mut game_board_shadow = [false; BOARD_N*BOARD_N];
    let mut game_line: usize = 0;
    let mut winning_board: bool = false;
    let mut winning_number: i32;
    // game stats are (turns_to_win, board_score)
    let mut game_stats: Vec<(i32, i32, i32)> = vec![];


    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                // Do sumthin
                if num_seq.len() == 0 {
                    for num in ip.split(',') {
                        num_seq.push(num.parse::<i32>().expect("Invalid input"));
                    }
                } else {
                    // reset game scan on newline
                    if ip.len() < 3 {
                        game_line = 0;
                    } 
                    else {
                        for (i, n) in ip.split_whitespace().enumerate() {
                            game_board[game_line * 5 + i] = n.parse::<i32>().expect("Invalid input");
                            game_board_shadow[game_line * 5 + i] = false;
                        }
                        game_line += 1;
                        // println!("game line {}", game_line);
                        if game_line >= BOARD_N {
                            // println!("end reading game");
                            for (i, n) in num_seq.iter().enumerate() {
                                for (it, m) in game_board.iter().enumerate() {
                                    if m == n {
                                        // println!("found match {}", m);
                                        game_board_shadow[it] = true;
                                        let mut h_win = true;
                                        let mut v_win = true;
                                        for j in 0..(BOARD_N-1) {
                                            if !game_board_shadow[j*5 + it%5] {
                                                v_win = false;
                                            }
                                            if !game_board_shadow[(it/5)*5 + j] {
                                                h_win = false;
                                            }
                                        }

                                        if h_win || v_win {
                                            winning_number = n.clone();
                                            winning_board = true;
                                            let mut score = 0;
                                            for (k, num) in game_board.iter().enumerate() {
                                                if !game_board_shadow[k] {
                                                    score += num;
                                                }
                                            }
                                            println!("won game in {} turns, score {}, with number {}", i, score, winning_number);

                                            game_stats.push((i as i32, score, winning_number));
                                            // calculate and save score in game_stats
                                        }
                                        // Move to next board
                                    }
                                    if winning_board { break }
                                }
                                if winning_board { break }
                            }
                            winning_board = false;
                            winning_number = 0;
                        } 
                    }
                }
            }
        }
    }

    // Go through scores to find winner with least number of turns
    // use board_score of winning board as final score
    let mut min_turns_to_win = 999;
    let mut r: i32 = 0;
    println!(" scorboard: {:?}",game_stats );
    for (turns_to_win, _, _) in game_stats.clone() {
        println!("{}", turns_to_win);
        if turns_to_win < min_turns_to_win {
            min_turns_to_win = turns_to_win;
        }
    }

    let mut best_score = 0;
    for (turns_to_win, board_score, winning_number) in game_stats.clone() {
        if turns_to_win == min_turns_to_win && best_score < board_score {
            best_score = board_score;
            println!("New best score {}\tturns to win {}\twinning number {}", 
                                best_score, min_turns_to_win, winning_number);
            r = winning_number*board_score;
        }
    }
    
    return r;
}

fn part2(filename: &str) -> i32{
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

