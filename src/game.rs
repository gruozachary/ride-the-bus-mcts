use crate::card;

#[derive(Clone, Copy)]
enum HiLo {
    Higher,
    Lower,
}

#[derive(Clone, Copy)]
enum InOut {
    Inside,
    Outside,
}

#[derive(Clone, Copy)]
enum State {
    Start,
    Stage1PlayerPicked(card::Colour),
    Stage1DealerPicked(card::Colour, card::Card),
    Stage2PlayerPicked(card::Card, HiLo),
    Stage2DealerPicked(card::Card, HiLo, card::Card),
    Stage3PlayerPicked(card::Card, card::Card, InOut),
    Stage3DealerPicked(card::Card, card::Card, InOut, card::Card),
    Stage4PlayerPicked(card::Card, card::Card, card::Card, card::Suit),
    Stage4DealerPicked(card::Card, card::Card, card::Card, card::Suit, card::Card),
    Finished(u32),
}
