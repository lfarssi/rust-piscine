use rand::Rng;
#[derive(Debug,PartialEq)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
    
}
#[derive(Debug,PartialEq)]
pub enum Rank {
    Ace,
    King,
    Jack,
    Queen,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let randm= rand::thread_rng().gen_range(0..4);
        match randm {
            0 => Suit::Spade,
            1 => Suit::Heart,
            2 => Suit::Diamond,
            _ => Suit::Club,
        }
        
    }

    pub fn translate(value: u8) -> Suit {
         match value {
            0 => Suit::Spade,
            1 => Suit::Heart,
            2 => Suit::Diamond,
            _ => Suit::Club,
        }
        
    }
}
impl Rank {
    pub fn random() -> Rank {
         let randm= rand::thread_rng().gen_range(0..5);
        match randm {
            0 => Rank::Ace,
            1 => Rank::King,
            2 => Rank::Jack,
            3 => Rank::Queen,
            _ => Rank::Number(randm),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            0 => Rank::Ace,
            1 => Rank::King,
            2 => Rank::Jack,
            3 => Rank::Queen,
            _ => Rank::Number(value),
        }
    }
}
#[derive(Debug,PartialEq)]

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    if card.suit == Suit::Spade && card.rank==Rank::Ace{
        return true;
    }
    return false;
}
