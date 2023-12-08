use std::collections::HashMap;

use reader::read_lines;
use regex::Regex;

fn main() {
    let regex = Regex::new(r"\d+\W(?:red|blue|green)\b").unwrap();
    let mut sum = 0;
    if let Ok(lines) = read_lines("./inputs/2.txt") {
        for line in lines {
            if let Ok(data) = line {
                let current_game_max = regex.find_iter(&data).fold(
                    HashMap::from([("red", 0), ("green", 0), ("blue", 0)]),
                    |mut acc, current| -> HashMap<&str, u32> {
                        let value_color: Vec<&str> = current.as_str().split(" ").collect();
                        let (value, color) =
                            (value_color[0].parse::<u32>().unwrap(), value_color[1]);
                        if value > *acc.get(color).unwrap() {
                            acc.insert(color, value);
                        }
                        acc
                    },
                );
                sum += current_game_max
                    .values()
                    .fold(1, |acc, current| acc * current)
            }
        }
    }
    println!("{}", sum)
}
