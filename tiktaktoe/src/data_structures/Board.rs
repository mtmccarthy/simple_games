use data_structures::Tile::Tile;
use data_structures::PlayerToken::PlayerToken;

#[derive(Debug, Clone)]
pub struct Board {
    board_state: Vec<Vec<Tile>>
}
/**
Initializes an empty 3x3 board
*/
pub fn init_board() -> Board{
    return Board{board_state: vec![make_empty_row(3); 3]}
}

fn make_empty_row(len: usize) -> Vec<Tile>{
    return vec![Tile {token: PlayerToken::NoToken}; len]
}

impl PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
          return self.board_state == other.board_state;
        }
}

pub fn place_token_board(board: &Board, token: PlayerToken, row_col: (usize, usize)) -> Result<Board, &'static str>{
    let row = row_col.0;
    let col = row_col.1;

    let mut new_board = board.clone();
    let mut tile = new_board.board_state[row][col].clone();
    match tile.update_tile(token) {
        Ok(_) => (),
        Err(_) => return Err("Cannot placed token")
    };
    new_board.board_state[row][col] = tile;
    return Ok(new_board);

}

#[cfg(test)]
mod tests {
    use data_structures::PlayerToken::PlayerToken;
    use data_structures::Tile::Tile;
    use data_structures::Board::init_board;
    use data_structures::Board::Board;
    use data_structures::Board::make_empty_row;
    use data_structures::Board::place_token_board;
    #[test]
    fn test_init() {
        let mt_tile = Tile {token: PlayerToken::NoToken};
        let mut row = Vec::new();
        row.push(mt_tile.clone());
        row.push(mt_tile.clone());
        row.push(mt_tile.clone());
        let second_row = row.clone();
        let third_row = row.clone();
        let mut board = Vec::new();
        board.push(row);
        board.push(second_row);
        board.push(third_row);
        assert_eq!(init_board(), Board{board_state: board})
    }

    #[test]
    fn test_place_token() {
        let mt_board = init_board();
        let row_col = (1, 2);
        let first_row = make_empty_row(3);
        let third_row = first_row.clone();
        let mt_tile = Tile {token: PlayerToken::NoToken};
        let placed_x_tile = Tile {token: PlayerToken::XToken};
        let placed_o_tile = Tile {token: PlayerToken::OToken};
        let second_row_x = vec![mt_tile.clone(), mt_tile.clone(), placed_x_tile];
        let second_row_o = vec![mt_tile.clone(), mt_tile.clone(), placed_o_tile];

        let placed_x_on_empty =
            Board {board_state: vec![first_row.clone(), second_row_x, third_row.clone()]};
        let placed_o_on_empty =
            Board {board_state: vec![first_row.clone(), second_row_o, third_row.clone()]};

        assert_eq!(Ok(placed_x_on_empty), place_token_board(&mt_board, PlayerToken::XToken, row_col));
        assert_eq!(Ok(placed_o_on_empty), place_token_board(&mt_board, PlayerToken::OToken, row_col));

    }
}