use data_structures::Card::{Card, make_number_card, Suit, Rank, get_suits, make_card};


#[derive(Debug)]
pub struct GameState {
    players: Vec<Player>,
    deck: Vec<Card>,
    turn_num: usize
}

#[derive(Debug)]
pub struct Player {
    score: usize,
    hand: Vec<Card>
}


pub fn init(num_players: usize) -> GameState {

    let deck = make_deck(52);
    let players = make_players(num_players);
    let shuffled_deck = shuffle_deck(deck);

    let num_cards_per_player = calc_num_cards_to_deal(num_players);

    return GameState {
        players,
        deck: shuffled_deck,
        turn_num: 0
    }

}

fn calc_num_cards_to_deal(num_players: usize) {

}

pub fn make_deck(deck_size: usize) -> Vec<Card> {

    let mut deck : Vec<Card> = vec![];

    let suites = get_suits();

    //Push Jacks
    for suit in suites {
        deck.push(make_card(Rank::JACK, suit.clone()));
        deck.push(make_card(Rank::QUEEN, suit.clone()));
        deck.push(make_card(Rank::KING, suit.clone()));
        deck.push(make_card(Rank::ACE, suit.clone()));


        for i in 2..11 {
            deck.push(make_number_card(i, suit.clone()));
        }
    }


    return deck;
}

pub fn make_players(num_players: usize) -> Vec<Player>{

    let mut players: Vec<Player> = vec![];

    for i in 0..num_players {
        players.push(make_player());
    }

    return players;
}

pub fn make_player() -> Player {
    return Player {
        score: 0,
        hand: vec![]
    }
}

pub fn shuffle_deck(deck: Vec<Card>) -> Vec<Card> {
    return deck;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {

        let init_deck2 = init(2);

        assert_eq!(0, init_deck2.turn_num);
        assert_eq!(52, init_deck2.deck.len());
        assert_eq!(2, init_deck2.players.len());

    }
}


