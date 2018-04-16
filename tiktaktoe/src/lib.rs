pub mod data_structures;
use data_structures::Board::init_board;
use data_structures::PlayerToken::PlayerToken;
use data_structures::Board::place_token_board;
use data_structures::GameState::GameState;
use data_structures::GameState::place_token;
use data_structures::GameState::init_game_state;
use data_structures::Player::Player;


pub fn test() {

    let init_gs = init_game_state();

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
