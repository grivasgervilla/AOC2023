use std::fs::File;
use std::cmp::max;
use std::io::{BufRead, BufReader};
use regex::Regex;

const RED_CUBE_MAX : u16 = 12;
const GREEN_CUBE_MAX : u16 = 13;
const BLUE_CUBE_MAX : u16 = 14;

struct GameData {
    game_id: u16,
    red_cubes : u16,
    green_cubes : u16,
    blue_cubes : u16
}

pub fn solver() -> Result<(), std::io::Error>{
    let reader = BufReader::new(File::open("input/2.txt")?);
    let mut valid_game_id_sum : u16 = 0;
    let mut power_game_sum : u32 = 0;

    for line in reader.lines(){
        let game_info = parse_line(line);

        if game_info.red_cubes <= RED_CUBE_MAX &&
            game_info.green_cubes <= GREEN_CUBE_MAX &&
            game_info.blue_cubes <= BLUE_CUBE_MAX {
            valid_game_id_sum += game_info.game_id;
        }

        power_game_sum += (game_info.red_cubes * game_info.green_cubes * game_info.blue_cubes) as u32;

    }

    println!("The sum of valid game ids is {valid_game_id_sum}.");
    println!("The sum of the power of each game is {power_game_sum}");

    Ok(())
}

fn parse_line(line : std::io::Result<String>) -> GameData {
    let current_line : String = line.unwrap().to_string();

    let game_id_extractor = Regex::new(r"Game ([0-9]+): (.*)").unwrap();
    let red_cube_extractor = Regex::new(r"([0-9]+) red").unwrap();
    let green_cube_extractor = Regex::new(r"([0-9]+) green").unwrap();
    let blue_cube_extractor = Regex::new(r"([0-9]+) blue").unwrap();

    let mut red_cube_count : u16 = 0;
    let mut green_cube_count : u16 = 0;
    let mut blue_cube_count : u16 = 0;

    let Some((_, [game_id, cube_data])) =
        game_id_extractor.captures(current_line.as_str()).map(|parts| parts.extract())
        else {todo!()};

    for extractions in cube_data.split(";"){
        match  red_cube_extractor.captures(extractions).map(|parts| parts.extract()){
            Some((_, [red_cubes])) => red_cube_count = max(red_cube_count, red_cubes.parse::<u16>().unwrap()),
            None => ()
        }
        match  green_cube_extractor.captures(extractions).map(|parts| parts.extract()){
            Some((_, [green_cubes])) => green_cube_count = max(green_cube_count, green_cubes.parse::<u16>().unwrap()),
            None => ()
        }
        match  blue_cube_extractor.captures(extractions).map(|parts| parts.extract()){
            Some((_, [blue_cubes])) => blue_cube_count = max(blue_cube_count, blue_cubes.parse::<u16>().unwrap()),
            None => ()
        }
    }
    
    return GameData{
        game_id: game_id.parse().unwrap(),
        red_cubes: red_cube_count,
        green_cubes: green_cube_count,
        blue_cubes: blue_cube_count
    }
}