use reader::read_lines;

fn extract_number(index: usize, row: &String) -> (u32, usize, usize) {
    let mut acc: Vec<u32> = row[index..index + 1]
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    let mut current_index = (index.clone() + 1) as i32;
    while let Some(potential_digit) = row
        .chars()
        .nth(current_index as usize)
        .and_then(|x| x.to_digit(10))
    {
        acc.insert(acc.len(), potential_digit);
        current_index += 1;
    }
    let max_index = current_index.clone();
    current_index = (index.clone() - 1) as i32;
    while let Some(potential_digit) = row
        .chars()
        .nth(current_index as usize)
        .and_then(|x| x.to_digit(10))
    {
        acc.insert(0, potential_digit);
        current_index -= 1;
    }
    (
        acc.iter()
            .enumerate()
            .map(|(index, x)| x * u32::pow(10, (acc.len() - 1) as u32 - index as u32))
            .sum(),
        (max_index - 1) as usize,
        (current_index.clone() + 1) as usize,
    )
}

fn get_adjacent_numbers(
    schematic: &Vec<String>,
    column_index: usize,
    row_index: usize,
) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    let to_check = [
        (column_index - 1, row_index),
        (column_index + 1, row_index),
        (column_index - 1, row_index - 1),
        (column_index, row_index - 1),
        (column_index + 1, row_index - 1),
        (column_index - 1, row_index + 1),
        (column_index, row_index + 1),
        (column_index + 1, row_index + 1),
    ];
    let mut already_checked: Vec<(usize, usize, usize)> = Vec::new();
    for coords in to_check {
        let (column_index, row_index) = coords;
        if let Some(_number) = schematic
            .get(row_index)
            .and_then(|row| row.get(column_index..column_index + 1))
            .and_then(|character| character.parse::<i32>().ok())
        {
            if already_checked
                .iter()
                .find(|y| y.0 == row_index && column_index >= y.1 && column_index <= y.2)
                .is_some()
            {
                continue;
            }
            let (extracted, max_index, min_index) =
                extract_number(column_index, schematic.get(row_index).unwrap());
            already_checked.push((row_index, min_index, max_index));
            result.push(extracted);
        }
    }
    result
}

fn main() {
    const SYMBOLS: [char; 10] = ['*', '/', '@', '&', '$', '=', '#', '-', '+', '%'];
    let mut sum = 0;
    let schematic = read_lines("./inputs/3.txt")
        .unwrap()
        .map(|lines| lines.unwrap())
        .collect::<Vec<String>>();

    for (i, line) in schematic.iter().enumerate() {
        for (j, character) in line.chars().enumerate() {
            if SYMBOLS.contains(&character) {
                let numbers = get_adjacent_numbers(&schematic, j, i);
                sum += numbers.iter().fold(0, |acc, current| acc + current)
            }
        }
    }
    println!("{}", sum)
}
