use data_structures::Card::Card;


#[derive(Debug)]
struct GameState {
    players: Vec<Player>,
    deck: Vec<Card>,
    turn_num: usize
}

#[derive(Debug)]
struct Player {
    score: usize,
    hand: Vec<Card>
}


