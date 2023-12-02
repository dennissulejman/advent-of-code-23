use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

const INPUT_FILE: &str = "../input.txt";

const ZERO: &str = "zero";
const ONE: &str = "one";
const TWO: &str = "two";
const THREE: &str = "three";
const FOUR: &str = "four";
const FIVE: &str = "five";
const SIX: &str = "six";
const SEVEN: &str = "seven";
const EIGHT: &str = "eight";
const NINE: &str = "nine";

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut result = 0;

    if let Ok(lines) = read_lines(INPUT_FILE) {
        for line in lines {
            if let Ok(current_line) = line {
                let mut first_num_as_char = '0';
                let mut last_num_as_char = '0';

                for c in current_line.chars() {
                    if c.is_numeric() {
                        first_num_as_char = c;
                        break;
                    }
                }

                for c in current_line.chars().rev() {
                    if c.is_numeric() {
                        last_num_as_char = c;
                        break;
                    }
                }

                let current_line_num: u32 = format!("{}{}", first_num_as_char, last_num_as_char)
                    .parse()
                    .unwrap();

                result += current_line_num;
            }
        }
    }

    println!("{}", result);
}

fn part_two() {
    let mut result = 0;

    if let Ok(lines) = read_lines(INPUT_FILE) {
        for line in lines {
            if let Ok(current_line) = line {
                let mut first_num_as_char = ' ';
                let mut last_num_as_char = ' ';

                let mut current_num_spelled_str = String::new();

                for c in current_line.chars() {
                    if c.is_numeric() {
                        first_num_as_char = c;
                        current_num_spelled_str = String::new();
                        break;
                    } else {
                        current_num_spelled_str.push(c);
                        if let Ok(num_char) = convert_to_num_str(current_num_spelled_str.as_str()) {
                            first_num_as_char = num_char;
                            current_num_spelled_str = String::new();
                            break;
                        }
                    }
                }

                for c in current_line.chars().rev() {
                    if c.is_numeric() {
                        last_num_as_char = c;
                        current_num_spelled_str.clear();
                        break;
                    } else {
                        current_num_spelled_str.insert(0, c);
                        if let Ok(num_char) = convert_to_num_str(current_num_spelled_str.as_str()) {
                            last_num_as_char = num_char;
                            current_num_spelled_str.clear();
                            break;
                        }
                    }
                }

                let current_line_num: u32 = format!("{}{}", first_num_as_char, last_num_as_char)
                    .parse()
                    .unwrap();

                result += current_line_num;
            }
        }
    }

    println!("{}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn convert_to_num_str(num_spelled_str: &str) -> Result<char, String> {
    match num_spelled_str {
        x if x.contains(ZERO) => Ok('0'),
        x if x.contains(ONE) => Ok('1'),
        x if x.contains(TWO) => Ok('2'),
        x if x.contains(THREE) => Ok('3'),
        x if x.contains(FOUR) => Ok('4'),
        x if x.contains(FIVE) => Ok('5'),
        x if x.contains(SIX) => Ok('6'),
        x if x.contains(SEVEN) => Ok('7'),
        x if x.contains(EIGHT) => Ok('8'),
        x if x.contains(NINE) => Ok('9'),
        &_ => Err(format!(
            "Could not convert {} into a number",
            num_spelled_str
        )),
    }
}
