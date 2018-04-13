use data_structures::Player::Player;
use data_structures::Board::Board;
use data_structures::Board::init_board;
use data_structures::PlayerToken::PlayerToken;
use data_structures::Board::place_token_board;

#[derive(Debug, Clone, PartialEq)]
struct GameState {
    players: (Player, Player),
    board: Board,
    turnNum: usize
}

fn init_game_state() -> GameState {
    return GameState {
        players: (Player, Player),
        board: init_board(),
        turnNum: 0
    }
}

fn place_token(gs: &GameState, token: PlayerToken, row_col: (usize, usize)) -> Result<GameState, &'static str> {
    let new_board = match place_token_board(&gs.board, token, row_col) {
        Ok(board) => board,
        Err(_) => return Err("There was an issue placing the token on the board")
    };
    return Ok(
        GameState {
            players: gs.players.clone(),
            board: new_board,
            turnNum: gs.turnNum + 1
    })
}

#[cfg(test)]
mod tests {
    use data_structures::Board::init_board;
    use data_structures::Board::place_token_board;
    use data_structures::PlayerToken::PlayerToken;
    use data_structures::GameState::init_game_state;
    use data_structures::GameState::place_token;
    #[test]
    fn test_place_token(){
        let mt_board = init_board();
        let row_col = (1,1);
        let token = PlayerToken::XToken;
        let expected_board = place_token_board(&mt_board, token.clone(), row_col).unwrap();


        let init_game_state = init_game_state();
        let update_game_state =
            place_token(&init_game_state, token.clone(), row_col).unwrap();
        assert_eq!(0, init_game_state.turnNum);
        assert_eq!(1, update_game_state.turnNum);

        assert_eq!(mt_board, init_game_state.board);
        assert_eq!(expected_board, update_game_state.board);

    }
}