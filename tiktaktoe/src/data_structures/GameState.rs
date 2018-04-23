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
    pub board: Board,
    pub turn_num: usize
}

impl Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player 1 is X. Player 2 is O\n");
        write!(f, "Turn Number: {}\n",  self.turn_num);
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
        turn_num: 0
    }
}

pub fn place_token(gs: GameState, token: &PlayerToken, row_col: (usize, usize)) -> Result<GameState, &'static str> {
    let new_board = match place_token_board(&gs.board, token.clone(), row_col) {
        Ok(board) => board,
        Err(_) => return Err("There was an issue placing the token on the board")
    };
    return Ok(
        GameState {
            players: gs.players.clone(),
            board: new_board,
            turn_num: gs.turn_num + 1
    })
}

pub fn is_game_over(gs: GameState, dst: (usize, usize), token: &PlayerToken) -> bool{
    let maybe_new_gs = place_token(gs, token, dst);

    match maybe_new_gs {
        Ok(game_state) => return is_victory(game_state, token),
        Err(_) => return false
    }
}

pub fn is_victory(gs: GameState, token: &PlayerToken) -> bool {
    print!("{}", &gs);
    let board = gs.board;
    //Check row wins

    let victory_condition =
        |all_same_token: bool, tile: &Tile| all_same_token && (tile.token == *token);

    let mut cols: Vec<Vec<Tile>> = vec![];

    let rows = &board.board_state.clone().iter().len();
    for x in 0..*rows{
        cols.push(vec![]);
    }
    let mut row_num = 0;
    let test_row_board = &board.board_state.clone();
    for row in test_row_board {
        let row_win = row.iter().fold(true, &victory_condition);
        if row_win {
            return true;
        }
        let mut col_num = 0;
        for tile in row {
            cols[col_num].push(*tile);
            col_num += 1;
        }

        row_num += 1;
    }
    //Check col win
    let test_col_board = &board.board_state.clone();
    for col in cols {
        let col_win = col.iter().fold(true, &victory_condition);
        if col_win {
            return true;
        }
    }

    //Check cross1 win
    let mut row_num = 0;
    let mut col_num = 0;
    let mut is_cross1_win = true;
    let mut test_cross1_board = &board.board_state.clone();
    for x in 0..*rows {

        row_num += 1;
        col_num += 1;
    }

    if is_cross1_win {
        return true;
    }


    let mut is_cross2_win = true;
    let mut test_cross2_board = &board.board_state.clone();
    row_num -= rows.clone();
    col_num = rows.clone();
    for x in 0..*rows {
        is_cross2_win &= test_cross2_board[row_num][col_num].token.clone() == *token;

        row_num -= 1;
        col_num -= 1;
    }

    if is_cross2_win {
        return true;
    }


    return false;

}



#[cfg(test)]
mod tests {
    use data_structures::Board::init_board;
    use data_structures::Board::place_token_board;
    use data_structures::PlayerToken::PlayerToken;
    use data_structures::GameState::init_game_state;
    use data_structures::GameState::place_token;
    use data_structures::GameState::is_victory;
    use data_structures::Tile::Tile;
    #[test]
    fn test_place_token(){
        let mt_board = init_board();
        let row_col = (1,1);
        let token = PlayerToken::XToken;
        let expected_board = place_token_board(&mt_board, token.clone(), row_col).unwrap();


        let init_game_state = init_game_state();
        let update_game_state =
            place_token(init_game_state.clone(), &token, row_col).unwrap();
        assert_eq!(0, init_game_state.turn_num);
        assert_eq!(1, update_game_state.turn_num);

        assert_eq!(mt_board, init_game_state.board);
        assert_eq!(expected_board, update_game_state.board);

    }

    #[test]
    fn test_display_gs() {

        assert!(true)
    }

    #[test]
    fn test_victory_condition() {
        let x_tile: Tile = Tile {token: PlayerToken::XToken};
        let no_tile: Tile = Tile {token: PlayerToken::NoToken};
        let row_win = vec![
            vec![x_tile.clone(), x_tile.clone(), x_tile.clone()],
            vec![no_tile.clone(), no_tile.clone(), no_tile.clone()],
            vec![no_tile.clone(), no_tile.clone(), no_tile.clone()]];
        let mut gs = init_game_state();
        gs.board.board_state = row_win;
        assert_eq!(true, is_victory(gs, &PlayerToken::XToken));

        let col_win = vec![
            vec![x_tile.clone(), no_tile.clone(), no_tile.clone()],
            vec![x_tile.clone(), no_tile.clone(), no_tile.clone()],
            vec![x_tile.clone(), no_tile.clone(), no_tile.clone()]];
        let mut gs = init_game_state();
        gs.board.board_state = col_win;
        assert_eq!(true, is_victory(gs, &PlayerToken::XToken));

        let cross1_win = vec![
            vec![x_tile.clone(), no_tile.clone(), no_tile.clone()],
            vec![no_tile.clone(), x_tile.clone(), no_tile.clone()],
            vec![no_tile.clone(), no_tile.clone(), x_tile.clone()]];

        let mut gs = init_game_state();
        gs.board.board_state = cross1_win;
        assert_eq!(true, is_victory(gs, &PlayerToken::XToken));

        let cross2_win = vec![
            vec![no_tile.clone(), no_tile.clone(), x_tile.clone()],
            vec![no_tile.clone(), x_tile.clone(), no_tile.clone()],
            vec![x_tile.clone(), no_tile.clone(), x_tile.clone()]];
        let mut gs = init_game_state();
        gs.board.board_state = cross2_win;
        assert_eq!(true, is_victory(gs, &PlayerToken::XToken));

    }
}