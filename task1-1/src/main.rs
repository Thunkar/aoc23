use reader::read_lines;

const VALID_NUMBERS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn map_numberic_literal(literal: &str) -> &str {
    match literal {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => literal,
    }
}

fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("./inputs/1.txt") {
        for line in lines {
            if let Ok(data) = line {
                let first_digit = VALID_NUMBERS
                    .iter()
                    .flat_map(|pat| data.match_indices(pat))
                    .min_by(|a, b| a.0.cmp(&b.0));
                let last_digit = VALID_NUMBERS
                    .iter()
                    .flat_map(|pat| data.rmatch_indices(pat))
                    .max_by(|a, b| a.0.cmp(&b.0));
                if let (Some(first_digit), Some(last_digit)) = (first_digit, last_digit) {
                    sum += (map_numberic_literal(first_digit.1).to_owned()
                        + map_numberic_literal(last_digit.1))
                    .parse::<i32>()
                    .unwrap();
                }
            }
        }
    }
    println!("{}", sum)
}
