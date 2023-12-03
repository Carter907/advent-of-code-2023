use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
// Determine which games would have been possible if the
// bag had been loaded with only
// 12 red cubes, 13 green cubes, and 14 blue cubes.
// What is the sum of the IDs of those games?

// e.g. 18 red, 15 blue, 12 green 
pub static CUBES: [i32; 3] = [12, 13, 14];

pub struct Reveal {
    pub game_id: usize,
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

pub fn check_reveal(reveal: &Reveal) -> bool {
    if reveal.red > CUBES[0] || reveal.green > CUBES[1] || reveal.blue > CUBES[2] {
        return false;
    }

    true
}

pub fn get_cube_reveals(line: &String) -> Vec<Reveal> {
    line
        .split(";")
        .map(|s| {
            let reveal_data = s.split_whitespace().map(|t| t.trim_matches(',')).collect::<Vec<&str>>();
            let mut game_id: usize = 0;
            let mut red: i32 = 0;
            let mut green: i32 = 0;
            let mut blue: i32 = 0;
            let mut previous_value: i32 = -1;

            for token in reveal_data {
                match token {
                    "red" => {
                        red = previous_value;
                    }
                    "green" => {
                        green = previous_value;
                    }
                    "blue" => {
                        blue = previous_value;
                    }
                    _ => {
                        let num = token.parse::<i32>();
                        match num {
                            Ok(_) => {
                                if previous_value == -1 {
                                    game_id = previous_value as usize;
                                }
                            }
                            Err(_) => {
                                continue;
                            }
                        }
                    }
                }
            }
            Reveal {
                game_id,
                red,
                green,
                blue,
            }
        }
        ).collect()
}

pub fn parse_input_data() -> Vec<String> {
    let file = File::open("input_file").expect("failed to read file");
    let mut buffer = BufReader::new(file);

    let mut lines = Vec::new();

    loop {
        let mut line = String::new();
        let bytes_read = buffer.read_line(&mut line).unwrap();
        if bytes_read == 0 {
            break;
        }
        lines.push(line);
    }
    lines
}