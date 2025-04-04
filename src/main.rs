use std::{
    io,
    sync::mpsc::{self, Receiver, Sender},
    thread, time,
};

use node::Node;

mod card;
mod game;
mod node;

fn main() {
    let mut buf = String::new();
    let mut root = Node::start();

    loop {
        let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();
        let new_root = root.clone();
        let thread_root = new_root.clone();

        thread::spawn(move || {
            Node::mcts(thread_root, rx);
        });

        for _ in 0..5 {
            thread::sleep(time::Duration::from_millis(5000));
            let best_moves = new_root.read().unwrap().get_best_moves();
            println!("{:?}", best_moves);
        }
        println!(
            "Best move: {:?}",
            new_root
                .read()
                .unwrap()
                .get_best_moves()
                .iter()
                .max_by(|(_, x), (_, y)| x.total_cmp(y))
                .unwrap()
        );
        tx.send(()).unwrap();

        let state = new_root.read().unwrap().state;

        buf.clear();
        println!("Current state: {:?}\nEnter move: ", state);
        io::stdin().read_line(&mut buf).unwrap();

        let search = state.apply_move(buf.trim().parse().unwrap()).unwrap();

        root = new_root
            .read()
            .unwrap()
            .children
            .iter()
            .find(|c| c.read().unwrap().state == search)
            .unwrap()
            .to_owned();
    }
}
