use std::cmp::max;
use std::collections::HashSet;
use std::fs::read_to_string;
use regex::Regex;

pub fn solver(){

    let re = Regex::new(r"(Card[\s]+[0-9]+:) ([^\|]*) (\|) ([^\|]*)").unwrap();
    let number_re = Regex::new(r"[0-9]+").unwrap();
    let mut points : u32 = 0;

    for line in read_to_string("input/4.txt").unwrap().lines(){
        let caps = re.captures(line).unwrap();

        let winning_numbers_info = caps.get(2).unwrap().as_str();
        let scratchcard_numbers_info = caps.get(4).unwrap().as_str();

        let winning_numbers : HashSet<_> = number_re.find_iter(winning_numbers_info)
            .map(|number| number.as_str()).collect();
        let scratchcard_numbers : HashSet<_> = number_re.find_iter(scratchcard_numbers_info)
            .map(|number| number.as_str()).collect();

        let match_numbers = winning_numbers.intersection(&scratchcard_numbers).count();

        if (match_numbers > 0) {
            points += 2_u32.pow(max(0, match_numbers as u32 - 1));
        }
    }

    println!("The total points are: {points}.")
}