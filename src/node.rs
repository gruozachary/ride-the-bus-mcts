use std::sync::{Arc, RwLock};

use crate::game::State;

struct Node {
    reward: u32,
    visits: u32,
    state: State,
    parent: Arc<RwLock<Node>>,
    children: Vec<Arc<RwLock<Node>>>,
}
impl Node {
    fn new(state: State, parent: Arc<RwLock<Node>>) -> Node {
        Node {
            reward: 0,
            visits: 0,
            state,
            parent,
            children: vec![],
        }
    }
}
