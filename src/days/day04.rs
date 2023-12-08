use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use regex::Regex;

pub fn solver(){

    let re = Regex::new(r"(Card[\s]+)([0-9]+): ([^\|]*) (\|) ([^\|]*)").unwrap();
    let number_re = Regex::new(r"[0-9]+").unwrap();
    let mut points : u32 = 0;

    let mut card_pile : HashMap<_, u32> = HashMap::new();

    let number_of_lines = read_to_string("input/4.txt").unwrap().lines().count();

    for line in read_to_string("input/4.txt").unwrap().lines(){
        let caps = re.captures(line).unwrap();

        let card_number : u32 = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let winning_numbers_info = caps.get(3).unwrap().as_str();
        let scratchcard_numbers_info = caps.get(5).unwrap().as_str();


        let winning_numbers : HashSet<_> = number_re.find_iter(winning_numbers_info)
            .map(|number| number.as_str()).collect();
        let scratchcard_numbers : HashSet<_> = number_re.find_iter(scratchcard_numbers_info)
            .map(|number| number.as_str()).collect();

        let match_numbers : u32 = winning_numbers.intersection(&scratchcard_numbers).count() as u32;

        if !card_pile.contains_key(&card_number) {
            card_pile.insert(card_number, 1);
        }
        else {
            card_pile.insert(card_number, card_pile.get(&card_number).unwrap() + 1);
        }


        if (match_numbers > 0) {
            points += 2_u32.pow(max(0, match_numbers as u32 - 1));

            for i in 1..match_numbers+1{
                if (card_number + i <= number_of_lines as u32){
                    if !card_pile.contains_key(&(card_number + i)) {
                        card_pile.insert(card_number + i, *card_pile.get(&card_number).unwrap());
                    }
                    else {
                        card_pile.insert(card_number + i, card_pile.get(&(card_number + i)).unwrap() + card_pile.get(&card_number).unwrap());
                    }
                }
            }
        }
    }

    let mut number_of_cards = 0;

    for card_copy_info in card_pile{
        number_of_cards += card_copy_info.1;
    }

    println!("The total points are: {points}.");
    println!("Totan number of scratchcards: {number_of_cards}");
}