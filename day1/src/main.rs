use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[allow(dead_code)]
fn calculate_calibration_value(line: &String) -> u32 {
    let mut calibration_value = 0;
    // this can be optimally with two pointers
    for chr in line.chars() {
        if let Some(first) = chr.to_digit(10) {
            calibration_value = 10 * first;
            break;
        }
    }
    for chr in line.chars().rev() {
        if let Some(last) = chr.to_digit(10) {
            calibration_value += last;
            break;
        }
    }
    calibration_value
}

fn calculate_calibration_value_complex(line: &String) -> u32 {
    let mut calibration_value = 0;
    let words = vec![
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let mut digits_pos_list: Vec<(usize, u32)> = vec![];
    // For each line find all occurences of each digit word and its idx then collect its value
    for dig_pair in words.iter() {
        let mut matches = line
            .match_indices(dig_pair.0)
            .map(|(idx, _)| (idx, dig_pair.1))
            .collect::<Vec<(usize, u32)>>();
        digits_pos_list.append(&mut matches);
    }

    let mut digs = line
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_digit(10))
        .map(|(idx, c)| (idx, c.to_digit(10).unwrap()))
        .collect::<Vec<(usize, u32)>>();

    digits_pos_list.append(&mut digs);

    digits_pos_list.sort();
    if let Some(p) = digits_pos_list.first() {
        calibration_value = p.1 * 10;
    }
    if let Some(p) = digits_pos_list.last() {
        calibration_value += p.1;
    }

    calibration_value
}

fn main() {
    let file_buffer = BufReader::new(File::open("input.txt").expect("Unable to open input file."));
    let result = file_buffer
        .lines()
        .filter_map(|buffer_line| buffer_line.ok())
        .fold(0_u32, |acc, line| {
            acc + calculate_calibration_value_complex(&line)
        });
    println!("{result}");
}
