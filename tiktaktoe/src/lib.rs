pub mod data_structures;
use data_structures::Board::init_board;
use data_structures::PlayerToken::PlayerToken;
use data_structures::Board::place_token_board;
use data_structures::GameState::GameState;
use data_structures::GameState::place_token;
use data_structures::GameState::init_game_state;

pub fn test() {
    let mt_board = init_board();
    let row_col = (1,1);
    let token = PlayerToken::XToken;
    let expected_board = place_token_board(&mt_board, token.clone(), row_col).unwrap();


    let init_game_state = init_game_state();
    let update_game_state =
        place_token(&init_game_state, token.clone(), row_col).unwrap();

    println!("{}", update_game_state);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
