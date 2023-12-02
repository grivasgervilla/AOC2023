use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};
use regex::{Error, Regex};

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

    for line in reader.lines(){
        let game_info = parse_line(line);

        if (game_info.red_cubes <= RED_CUBE_MAX &&
            game_info.green_cubes <= GREEN_CUBE_MAX &&
            game_info.blue_cubes <= BLUE_CUBE_MAX) {
            valid_game_id_sum += game_info.game_id;
        }
    }

    println!("The sum of valid game ids is {valid_game_id_sum}.");

    Ok(())
}

fn parse_line(line : std::io::Result<String>) -> GameData {
    
    return GameData{
        game_id: 0,
        red_cubes: 0,
        green_cubes: 0,
        blue_cubes: 0
    }
}