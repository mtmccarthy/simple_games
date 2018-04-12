use data_structures::Player::Player;
use data_structures::Board::Board;

#[derive(Debug)]
struct GameState {
    players: (Player, Player),
    board: Board
}