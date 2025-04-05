use std::{
    f64::consts::SQRT_2,
    sync::{
        Arc, RwLock,
        atomic::{self, AtomicBool},
    },
};

use crate::game::{Move, State};

pub struct Node {
    reward: f64,
    visits: f64,
    mov: Option<Move>,
    pub state: State,
    parent: Option<Arc<RwLock<Node>>>,
    pub children: Vec<Arc<RwLock<Node>>>,
}
impl Node {
    pub fn new(
        previous_state: State,
        mov: Move,
        parent: Arc<RwLock<Node>>,
    ) -> Option<Arc<RwLock<Node>>> {
        previous_state.apply_move(mov).map(|state| {
            Arc::new(RwLock::new(Node {
                reward: 0.0,
                visits: 0.0,
                mov: Some(mov),
                state,
                parent: Some(parent),
                children: vec![],
            }))
        })
    }

    pub fn start() -> Arc<RwLock<Node>> {
        Arc::new(RwLock::new(Node {
            reward: 0.0,
            visits: 0.0,
            mov: None,
            state: State::Start,
            parent: None,
            children: vec![],
        }))
    }

    fn score_node(&self, root_visits: f64) -> f64 {
        (self.reward / self.visits) + SQRT_2 * ((root_visits.ln() / self.visits).sqrt())
    }

    fn choose_best_child(&self, root_visits: f64) -> Arc<RwLock<Node>> {
        self.children
            .iter()
            .max_by(|x, y| {
                x.read()
                    .unwrap()
                    .score_node(root_visits)
                    .total_cmp(&y.read().unwrap().score_node(root_visits))
            })
            .unwrap()
            .to_owned()
    }

    fn select_node(root: Arc<RwLock<Node>>) -> Arc<RwLock<Node>> {
        let mut node = root.clone();

        while !node.read().unwrap().state.is_terminal() && !node.read().unwrap().children.is_empty()
        {
            let next = node
                .read()
                .unwrap()
                .choose_best_child(root.read().unwrap().visits);
            node = next;
        }

        node
    }

    fn expand(node: Arc<RwLock<Node>>) -> Arc<RwLock<Node>> {
        let possible_moves = node.read().unwrap().state.get_valid_moves();

        let state = node.read().unwrap().state;
        let children = &mut node.write().unwrap().children;
        for mov in possible_moves {
            children.push(Node::new(state, mov, node.clone()).unwrap());
        }

        children[0].clone()
    }

    fn backpropagate(node: Arc<RwLock<Node>>, reward: f64) {
        let mut maybe_node = Some(node.clone());
        while let Some(current_node) = maybe_node.clone() {
            let mut current_node_aq = current_node.write().unwrap();
            current_node_aq.visits += 1.0;
            current_node_aq.reward += reward;
            maybe_node = current_node_aq.parent.clone()
        }
    }

    pub fn mcts(root: Arc<RwLock<Node>>, stop: Arc<AtomicBool>) {
        let mut rng = rand::rng();

        while !stop.load(atomic::Ordering::Acquire) {
            let mut node = root.clone();

            node = Node::select_node(node);

            if !node.read().unwrap().state.is_terminal() {
                node = Node::expand(node);
            }

            let reward = node.read().unwrap().state.playout(&mut rng) as f64 / 20.0;

            Node::backpropagate(node, reward);
        }
    }

    pub fn get_best_moves(&self) -> Vec<(Move, f64)> {
        self.children
            .iter()
            .filter_map(|c| {
                c.read()
                    .unwrap()
                    .mov
                    .map(|mov| (mov, c.read().unwrap().visits / self.visits))
            })
            .collect()
    }

    pub fn find_child(node: Arc<RwLock<Node>>, mov: Move) -> Option<Arc<RwLock<Node>>> {
        node.read()
            .unwrap()
            .children
            .iter()
            .find(|c| c.read().unwrap().mov == Some(mov))
            .cloned()
    }
}
