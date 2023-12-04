use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

// The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

const INPUT_FILE: &str = "../input.txt";

const GAME: &str = "Game";

const BLUE: &str = "blue";
const GREEN: &str = "green";
const RED: &str = "red";

const MAX_BLUE_CUBES: u32 = 14;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_RED_CUBES: u32 = 12;

fn main() {
    if let Ok(lines) = read_lines(INPUT_FILE) {
        let mut result_part_one = 0;
        let mut result_part_two = 0;
        for line in lines {
            if let Ok(current_line) = line {
                let mut game_id = 0;
                let mut blue_cubes_highest_amount = 0;
                let mut green_cubes_highest_amount = 0;
                let mut red_cubes_highest_amount = 0;

                for s in current_line.split([':', ',', ';']) {
                    let values = s
                        .split(' ')
                        .skip_while(|&x| x.is_empty())
                        .collect::<Vec<&str>>();
                    let first = values[0];
                    let second = values[1];

                    match s {
                        x if x.contains(GAME) => {
                            game_id = second.parse().unwrap();
                        }
                        x if x.contains(BLUE) => {
                            let num = first.parse().unwrap();
                            if num > blue_cubes_highest_amount {
                                blue_cubes_highest_amount = num;
                            }
                        }
                        x if x.contains(GREEN) => {
                            let num: u32 = first.parse().unwrap();
                            if num > green_cubes_highest_amount {
                                green_cubes_highest_amount = first.parse().unwrap();
                            }
                        }
                        x if x.contains(RED) => {
                            let num: u32 = first.parse().unwrap();
                            if num > red_cubes_highest_amount {
                                red_cubes_highest_amount = first.parse().unwrap();
                            }
                        }
                        &_ => println!("No match"),
                    }
                }

                if blue_cubes_highest_amount <= MAX_BLUE_CUBES
                    && green_cubes_highest_amount <= MAX_GREEN_CUBES
                    && red_cubes_highest_amount <= MAX_RED_CUBES
                {
                    result_part_one += game_id;
                }

                result_part_two += blue_cubes_highest_amount
                    * green_cubes_highest_amount
                    * red_cubes_highest_amount;
            }
        }
        println!("{}", result_part_one);
        println!("{}", result_part_two);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
