use data_structures::Player::Player;
use data_structures::Board::Board;
use data_structures::Board::init_board;
use data_structures::PlayerToken::PlayerToken;
use data_structures::Board::place_token_board;
use std::fmt;
use std::fmt::Display;
use data_structures::Tile::Tile;

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    players: (Player, Player),
    board: Board,
    turnNum: usize
}

impl Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player 1 is X. Player 2 is O\n");
        write!(f, "Turn Number: {}\n",  self.turnNum);
        for row in &self.board.board_state {
            fmt_row(f, &row);
        }
        write!(f, "")
    }

}

fn fmt_row(f: &mut fmt::Formatter, row: &Vec<Tile>) {
    let mut row_display_line: String = String::new();
    for tile in row {
        match tile.token {
            PlayerToken::XToken => row_display_line += "|X",
            PlayerToken::OToken => row_display_line += "|O",
            PlayerToken::NoToken => row_display_line += "| ",
        };
    }
    row_display_line += "|\n";
    write!(f, "{}", row_display_line);
}


pub fn init_game_state() -> GameState {
    return GameState {
        players: (Player, Player),
        board: init_board(),
        turnNum: 0
    }
}

pub fn place_token(gs: &GameState, token: PlayerToken, row_col: (usize, usize)) -> Result<GameState, &'static str> {
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

    #[test]
    fn test_display_gs() {

        assert!(true)
    }
}