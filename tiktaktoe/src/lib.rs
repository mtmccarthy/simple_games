extern crate regex;
pub mod data_structures;
use data_structures::Board::init_board;
use data_structures::PlayerToken::PlayerToken;
use data_structures::Board::place_token_board;
use data_structures::GameState::GameState;
use data_structures::GameState::place_token;
use data_structures::GameState::init_game_state;
use data_structures::Player::Player;
use data_structures::GameState::is_game_over;
use std::io;
use regex::Regex;

pub fn test() {

    let mut current_gs = init_game_state();

    while true{

        let is_player_one_turn = & current_gs.turn_num % 2 == 0;
        if is_player_one_turn{
            //Player 1's Turn
            print!("Player 1's Turn. Enter tile to place as a row column pair. ex. \"1 1\" Represents the middle tile.\n\n");
            print!("{}", current_gs);
        }
        else {
            print!("Player 2's Turn. Enter tile to place as a row column pair. ex. \"1 1\" Represents the middle tile.\n\n");
        }
        print!("{}", current_gs);
        let row_col = wait_for_input();
        let player_token = match is_player_one_turn {
            true => PlayerToken::XToken,
            false => PlayerToken::OToken
        };

        match place_token(current_gs.clone(), &player_token, row_col) {
            Ok(gs) => {
                if is_game_over(current_gs.clone(), &player_token, row_col) {
                    current_gs = gs;
                    let player_to_congrats = match is_player_one_turn {
                        true => "Player 1",
                        false => "Player 2"
                    };
                    println!("Congratulations to {}", player_to_congrats);
                    println!("{}", current_gs);
                    return;
                }
                else {
                    current_gs = gs;
                }

            },
            Err(_) => println!("Not a valid move, please try again.")
        }


    }

}

fn wait_for_input() -> (usize, usize){
    let re = Regex::new(r"\d \d.*").unwrap();
    while true {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) =>
                match parse_input(&*input) {
                    Ok(result) => return result,
                    Err(msg) => println!("Could not be parsed. Please try again: {}\n", msg)
                }
            Err(_) => println!("Please try again\n")
        }

        println!("You typed: {}\n", input.trim());

    }

    return (0, 0)

    }


fn parse_input(input: &str) -> Result<(usize, usize), String> {

    let re = Regex::new(r"(\d) (\d)").unwrap();
    if re.is_match(input) {
        let groups = re.captures(input);
        match groups {
            Some(groups) =>{
                let row = match groups[1].parse::<usize>(){
                    Ok(row) => row,
                    Err(msg) => return Err(format!("Cannot parse row because {}\n", msg ))
                };
                let col = match groups[2].parse::<usize>(){
                    Ok(col) => col,
                    Err(msg) => return Err(format!("Cannot parse column because {}\n", msg ))
                };
                return Ok((row, col))
            },

            None => return Err(format!("Something went wrong with the grouping of the regex\n"))
        };

    }
    else {
        return Err(format!("Please enter a digit followed by a space followed by another digit. I.e. \"0 0\" To represent the upper first most tile"))
    }

    return Err(format!("Should never get here"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
