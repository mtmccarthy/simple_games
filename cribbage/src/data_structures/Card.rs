
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: Rank
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Suit {
    SPADES,
    HEARTS,
    CLUBS,
    DIAMONDS
}


#[derive(Debug, PartialOrd, PartialEq)]
pub enum Rank {
    NUMBER {value: usize},
    JACK,
    QUEEN,
    KING,
    ACE
}

impl Rank {
    pub fn value(&self) -> usize{
        match *self {
            Rank::NUMBER {value} => value,
            Rank::JACK => 10,
            Rank::QUEEN => 10,
            Rank::KING => 10,
            Rank::ACE => 1
        }
    }
}

pub fn make_number_rank(value: usize) -> Rank{
    return Rank::NUMBER {value};
}


pub fn make_card(rank: Rank, suit: Suit) -> Card {
    return Card {
        rank,
        suit
    }
}

pub fn get_suits() -> Vec<Suit> {
    return vec![Suit::SPADES, Suit::HEARTS, Suit::CLUBS, Suit::DIAMONDS];
}

pub fn make_number_card(value: usize, suit: Suit) -> Card {
    return Card {
        rank: make_number_rank(value),
        suit
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_number_rank() {
        assert_eq!(Rank::NUMBER {value: 3}, make_number_rank(3));
    }

    #[test]
    fn test_get_card_value() {
        let spades = Suit::SPADES;
        assert_eq!(7, Card{suit: Suit::SPADES, rank: make_number_rank(7)}.rank.value());

        assert_eq!(10, Card {suit: Suit::SPADES, rank: Rank::JACK}.rank.value());
        assert_eq!(10, Card {suit: Suit::SPADES, rank: Rank::QUEEN}.rank.value());
        assert_eq!(10, Card {suit: Suit::SPADES, rank: Rank::KING}.rank.value());
        assert_eq!(1, Card {suit: Suit::SPADES, rank: Rank::ACE}.rank.value());
    }

    #[test]
    fn test_make_number_card() {
        let test_number_card = Card {
            suit: Suit::SPADES,
            rank: Rank::NUMBER {
                value: 4
            }
        };

        assert_eq!(test_number_card, make_number_card(4, Suit::SPADES));
    }
}