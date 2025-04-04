#[derive(Clone, Copy)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
impl Suit {
    fn colour(&self) -> Colour {
        match self {
            Suit::Hearts => Colour::Red,
            Suit::Diamonds => Colour::Red,
            Suit::Clubs => Colour::Black,
            Suit::Spades => Colour::Black,
        }
    }
}

#[derive(Clone, Copy)]
enum Colour {
    Red,
    Black,
}

#[derive(Clone, Copy)]
enum Value {
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
}

#[derive(Clone, Copy)]
struct Card {
    suit: Suit,
    value: Value,
}
impl Card {
    fn new(suit: Suit, value: Value) -> Self {
        Card { suit, value }
    }
}
