use std::fmt::{Display, write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
impl Suit {
    pub fn colour(&self) -> Colour {
        match self {
            Suit::Hearts => Colour::Red,
            Suit::Diamonds => Colour::Red,
            Suit::Clubs => Colour::Black,
            Suit::Spades => Colour::Black,
        }
    }
}
impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Clubs => write!(f, "Clubs"),
            Suit::Spades => write!(f, "Spades"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Colour {
    Red,
    Black,
}
impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Colour::Red => write!(f, "Red"),
            Colour::Black => write!(f, "Black"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Two => write!(f, "Two"),
            Value::Three => write!(f, "Three"),
            Value::Four => write!(f, "Four"),
            Value::Five => write!(f, "Five"),
            Value::Six => write!(f, "Six"),
            Value::Seven => write!(f, "Seven"),
            Value::Eight => write!(f, "Eight"),
            Value::Nine => write!(f, "Nine"),
            Value::Ten => write!(f, "Ten"),
            Value::Jack => write!(f, "Jack"),
            Value::Queen => write!(f, "Queen"),
            Value::King => write!(f, "King"),
            Value::Ace => write!(f, "Ace"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}
impl Card {
    pub const fn new(suit: Suit, value: Value) -> Self {
        Card { suit, value }
    }

    pub fn rest_of_deck(cards: &[Self]) -> Vec<Self> {
        DECK.iter()
            .filter(|card| !cards.contains(card))
            .copied()
            .collect()
    }
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}

static DECK: [Card; 52] = [
    Card::new(Suit::Hearts, Value::Two),
    Card::new(Suit::Hearts, Value::Three),
    Card::new(Suit::Hearts, Value::Four),
    Card::new(Suit::Hearts, Value::Five),
    Card::new(Suit::Hearts, Value::Six),
    Card::new(Suit::Hearts, Value::Seven),
    Card::new(Suit::Hearts, Value::Eight),
    Card::new(Suit::Hearts, Value::Nine),
    Card::new(Suit::Hearts, Value::Ten),
    Card::new(Suit::Hearts, Value::Jack),
    Card::new(Suit::Hearts, Value::Queen),
    Card::new(Suit::Hearts, Value::King),
    Card::new(Suit::Hearts, Value::Ace),
    Card::new(Suit::Diamonds, Value::Two),
    Card::new(Suit::Diamonds, Value::Three),
    Card::new(Suit::Diamonds, Value::Four),
    Card::new(Suit::Diamonds, Value::Five),
    Card::new(Suit::Diamonds, Value::Six),
    Card::new(Suit::Diamonds, Value::Seven),
    Card::new(Suit::Diamonds, Value::Eight),
    Card::new(Suit::Diamonds, Value::Nine),
    Card::new(Suit::Diamonds, Value::Ten),
    Card::new(Suit::Diamonds, Value::Jack),
    Card::new(Suit::Diamonds, Value::Queen),
    Card::new(Suit::Diamonds, Value::King),
    Card::new(Suit::Diamonds, Value::Ace),
    Card::new(Suit::Clubs, Value::Two),
    Card::new(Suit::Clubs, Value::Three),
    Card::new(Suit::Clubs, Value::Four),
    Card::new(Suit::Clubs, Value::Five),
    Card::new(Suit::Clubs, Value::Six),
    Card::new(Suit::Clubs, Value::Seven),
    Card::new(Suit::Clubs, Value::Eight),
    Card::new(Suit::Clubs, Value::Nine),
    Card::new(Suit::Clubs, Value::Ten),
    Card::new(Suit::Clubs, Value::Jack),
    Card::new(Suit::Clubs, Value::Queen),
    Card::new(Suit::Clubs, Value::King),
    Card::new(Suit::Clubs, Value::Ace),
    Card::new(Suit::Spades, Value::Two),
    Card::new(Suit::Spades, Value::Three),
    Card::new(Suit::Spades, Value::Four),
    Card::new(Suit::Spades, Value::Five),
    Card::new(Suit::Spades, Value::Six),
    Card::new(Suit::Spades, Value::Seven),
    Card::new(Suit::Spades, Value::Eight),
    Card::new(Suit::Spades, Value::Nine),
    Card::new(Suit::Spades, Value::Ten),
    Card::new(Suit::Spades, Value::Jack),
    Card::new(Suit::Spades, Value::Queen),
    Card::new(Suit::Spades, Value::King),
    Card::new(Suit::Spades, Value::Ace),
];
