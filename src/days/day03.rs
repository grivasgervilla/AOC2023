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

    let regex_numbers = Regex::new(r"[0-9]+").unwrap();
    let regex_asterisk = Regex::new(r"[*]");
    let mut part_number_sum : u32 = 0;
    let mut gear_ratio_sum : u32 = 0;

    for (line_pos, line) in data.iter().enumerate(){
       for number in regex_numbers.find_iter(&*line){
           if is_part_num(data.clone(), line_pos, number.start(), number.end()) {
               part_number_sum += number.as_str().parse::<u32>().unwrap();
           }
       }

        for asterisk in regex_asterisk.as_ref().unwrap().find_iter(&*line){
           gear_ratio_sum += get_gear_ratio(data.clone(), line_pos, asterisk.start());
        }
    }

    println!("The part number sum is {part_number_sum}.");
    println!("The gear ratio sum is {gear_ratio_sum}.");


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

fn get_gear_ratio(data : Vec<String>, line_number : usize, asterisk_start : usize) -> u32{
    let prev_line_number = if line_number > 0 {line_number - 1} else {0};
    let next_line_number = if line_number == data.len() - 1 {line_number} else {line_number + 1};

    let pos_range_inf = if asterisk_start > 0 {asterisk_start - 1} else {0};
    let pos_range_sup = if asterisk_start == data[0].len() - 1 {asterisk_start} else {asterisk_start + 1};

    let regex_numbers = Regex::new(r"[0-9]+").unwrap();

    let mut contiguous_numbers : Vec<u32> = Vec::new();

    for number in regex_numbers.find_iter(&*data[prev_line_number]){
        if is_contiguous(pos_range_inf, pos_range_sup, number.start(), number.end() - 1) {
            contiguous_numbers.push(number.as_str().parse::<u32>().unwrap());
        }
    }

    for number in regex_numbers.find_iter(&*data[line_number]){
        if is_contiguous(pos_range_inf, pos_range_sup, number.start(), number.end() - 1) {
            contiguous_numbers.push(number.as_str().parse::<u32>().unwrap());
        }
    }

    for number in regex_numbers.find_iter(&*data[next_line_number]){
        if is_contiguous(pos_range_inf, pos_range_sup, number.start(), number.end() - 1) {
            contiguous_numbers.push(number.as_str().parse::<u32>().unwrap());
        }
    }

    if contiguous_numbers.len() == 2 { contiguous_numbers[0] * contiguous_numbers[1]  } else {0}
}

fn is_contiguous(asterisk_range_inf : usize, asterisk_range_sup : usize,
                    number_range_inf : usize, number_range_sup : usize) -> bool{

    !(number_range_sup < asterisk_range_inf || number_range_inf > asterisk_range_sup)
}