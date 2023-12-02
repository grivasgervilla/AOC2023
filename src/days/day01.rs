use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn solver() -> Result<(), std::io::Error>{

    let file = File::open("input/1.txt")?;
    let reader = BufReader::new(file);
    let mut first_sum: u32 = 0;
    let mut second_sum: u32 = 0;
    let mut line_number : u32 = 0;
    let mut contador : u32 = 0;

    let lines : Vec<_> = reader.lines().collect();

    for line in lines{
        let mut first_digit : u32 = 0;
        let mut last_digit : u32 = 0;

        for character in line.as_ref().unwrap().chars(){
            if character.is_numeric() {
                if first_digit == 0 {
                    first_digit = character.to_digit(10).unwrap();
                    last_digit = first_digit;
                }
                else {
                    last_digit = character.to_digit(10).unwrap();
                }
            }
        }

        first_sum += first_digit * 10 + last_digit;

        line_number = get_line_digit(line.unwrap().as_str());
        second_sum += line_number;
        contador += 1;
    }

    println!("The first sum is {first_sum}.");
    println!("The second sum is {second_sum}.");

    Ok(())
}

fn get_line_digit(word : &str) -> u32{
    let extracted_digits : Vec<&str> = get_line_digits(word);

    let first_digit : u32 = str_to_number(extracted_digits[0]);
    let second_digit: u32 = str_to_number(extracted_digits.last().unwrap());

    let line_number = first_digit * 10 + second_digit;

    return line_number;
}

fn get_line_digits(word : &str) -> Vec<&str>{
    let re = Regex::new(r"[1-9]|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)").unwrap();
    let mut line_digits : Vec<&str> = Vec::new();

    for offset in 0..word.len(){
        let result = re.find_at(word, offset);

        if result.is_some() { line_digits.push(result.unwrap().as_str())}
    }

    return line_digits;
}

fn str_to_number(string: &str) -> u32{
    return match string {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => 0
    }
}