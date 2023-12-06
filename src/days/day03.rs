use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn solver() -> Result<(), std::io::Error>{
    let mut data : Vec<String> = Vec::new();
    let reader = BufReader::new(File::open("input/3.txt")?);

    for line in reader.lines(){
        data.push(line.unwrap());
    }

    let number_of_lines = data.len();
    let number_of_chars_per_line = data[0].len();
    let re = Regex::new(r"[0-9]+").unwrap();
    let mut part_number_sum : u32 = 0;

    for (line_pos, line) in data.iter().enumerate(){
       for number in re.find_iter(&*line){
           if is_part_num(data.clone(), line_pos, number.start(), number.end()) {
               part_number_sum += number.as_str().parse::<u32>().unwrap();
           }
           //println!("{} en la posicion {}", number.as_str(), number.start());
       }
    }

    println!("The part number sum is {part_number_sum}.");


    Ok(())
}

fn is_part_num(data : Vec<String>, line_number : usize, number_start : usize, number_end : usize) -> bool{
    let mut is_part : bool = false;

    let re = Regex::new(r"[^.0-9]").unwrap();

    let begin_pos = if number_start > 0 {number_start - 1} else {0};
    let end_pos = min(number_end + 1, data[0].len());

    let prev_line_number = if line_number > 0 {line_number - 1} else {0};
    let next_line_number = if line_number == data.len() - 1 {line_number} else {line_number + 1};

    let substring_above = &(data[prev_line_number].clone())[begin_pos..end_pos];
    let substring_arround = &(data[line_number].clone())[begin_pos..end_pos];
    let substring_below = &(data[next_line_number].clone())[begin_pos..end_pos];

    re.is_match(substring_above) ||
        re.is_match(substring_arround) ||
        re.is_match(substring_below)
}