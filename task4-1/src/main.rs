use reader::read_lines;
use regex::Regex;

fn main() {
    let mut sum = 0;
    let winning_pattern = Regex::new(r"(?:(?:\s?\d+\s?)+)\s+\|").unwrap();
    let played_pattern = Regex::new(r"\|\s+(?:(?:\s?\d+\s?)+)\s?").unwrap();
    if let Ok(lines) = read_lines("./inputs/4.txt") {
        for line in lines {
            if let Ok(data) = line {
                let winning_str = winning_pattern.find(&data).unwrap().as_str();
                let winning_numbers: Vec<_> = winning_str[..winning_str.len() - 1]
                    .split(" ")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.trim().parse::<u32>().unwrap())
                    .collect();
                let played_str = played_pattern.find(&data).unwrap().as_str();
                let played_numbers: Vec<_> = played_str[1..played_str.len()]
                    .split(" ")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.trim().parse::<u32>().unwrap())
                    .collect();
                let value = winning_numbers
                    .iter()
                    .filter(|winning_number| {
                        played_numbers
                            .iter()
                            .find(|played_number| played_number == winning_number)
                            .is_some()
                    })
                    .collect::<Vec<_>>()
                    .len();
                if value > 0 {
                    sum += (2 as u32).pow(value as u32 - 1);
                }
            }
        }
    }
    println!("{}", sum)
}
