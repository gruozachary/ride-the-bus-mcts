use std::io;

use game::{Move, State};

mod card;
mod game;

fn main() {
    let mut game = State::Start;
    let stdin = io::stdin();
    let mut buf = String::new();

    let x: Move = "red".parse().unwrap();
    println!("{:?}", x);

    loop {
        buf.clear();
        println!("Current state: {:?}\nMake a move:", game);
        stdin.read_line(&mut buf).unwrap();
        if let Ok(mov) = buf.trim().parse() {
            game = game.apply_move(mov).unwrap();
        }
    }
}
