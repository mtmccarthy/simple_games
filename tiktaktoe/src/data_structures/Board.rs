use data_structures::Tile::Tile;
use data_structures::PlayerToken::PlayerToken;
#[derive(Debug)]
pub struct Board {
    board_state: Vec<Vec<Tile>>
}
/**
Initializes an empty 3x3 board
*/
fn init_board() -> Board{
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

#[cfg(test)]
mod tests {
    use data_structures::PlayerToken::PlayerToken;
    use data_structures::Tile::Tile;
    use data_structures::Board::init_board;
    use data_structures::Board::Board;
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
}