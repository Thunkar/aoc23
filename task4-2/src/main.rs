use std::collections::HashMap;

use reader::read_lines;
use regex::Regex;

fn main() {
    let mut _sum = 0;
    let mut cards: HashMap<u32, u32> = HashMap::new();
    let id_pattern = Regex::new(r"Card\s+(\d+)").unwrap();
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
                let card_id = id_pattern.captures(&data).unwrap()[1]
                    .parse::<u32>()
                    .unwrap();
                if !cards.contains_key(&card_id) {
                    cards.insert(card_id, 1);
                }
                let copies = cards.get(&card_id).unwrap().clone();
                for i in (card_id + 1)..(card_id + 1 + value as u32) {
                    let current_copies = cards.get(&i).or(Some(&1)).unwrap();
                    cards.insert(i, current_copies + copies);
                }
            }
        }
    }
    _sum = cards.values().sum();
    println!("{}", _sum)
}
