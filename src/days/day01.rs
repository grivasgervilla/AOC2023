use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solver() -> Result<(), std::io::Error>{

    let file = File::open("input/1.txt")?;
    let reader = BufReader::new(file);
    let mut sum : u32 = 0;

    for line in reader.lines(){
        let mut first_digit : u32 = 0;
        let mut last_digit : u32 = 0;

        for character in line.unwrap().chars(){
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

        sum += first_digit * 10 + last_digit;
    }

    println!("The sum is {sum}.");

    Ok(())
}