mod card;
mod game;
mod node;

use std::{
    io,
    sync::{
        Arc, RwLock,
        atomic::{self, AtomicBool},
    },
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{Event, KeyCode, poll, read};
use game::Move;
use itertools::Itertools;
use node::Node;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};
use tui_textarea::TextArea;

struct App<'a> {
    root: Arc<RwLock<Node>>,
    current_input: TextArea<'a>,
    poll_time: Duration,
    last_attempt: Instant,
    best_moves: Vec<(Move, f64)>,
    stop_mcts: Arc<AtomicBool>,
    exit: bool,
}
impl<'a> App<'a> {
    fn new(root: Arc<RwLock<Node>>) -> App<'a> {
        App {
            root,
            current_input: TextArea::default(),
            poll_time: Duration::from_millis(500),
            last_attempt: Instant::now(),
            best_moves: vec![],
            stop_mcts: Arc::new(AtomicBool::new(false)),
            exit: false,
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.start_mcts_thread();

        while !self.exit {
            if self.last_attempt.elapsed() > self.poll_time {
                self.best_moves = self.get_best_moves(5);
                self.last_attempt = Instant::now();
            }
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn get_best_moves(&self, number: usize) -> Vec<(Move, f64)> {
        self.root
            .read()
            .unwrap()
            .get_best_moves()
            .iter()
            .k_largest_by(number, |(_, x), (_, y)| x.total_cmp(y))
            .copied()
            .collect()
    }

    fn start_mcts_thread(&mut self) {
        let root = self.root.clone();
        let stop = self.stop_mcts.clone();
        thread::spawn(move || {
            Node::mcts(root, stop.clone());
            stop.store(false, atomic::Ordering::Release);
        });
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Ok(true) = poll(Duration::from_secs(0)) {
            if let Event::Key(key) = read()? {
                if key.code == KeyCode::Esc {
                    self.exit = true;
                } else if key.code == KeyCode::Enter {
                    if self.try_set_new_root() {
                        self.stop_mcts.store(true, atomic::Ordering::Release);
                        while self.stop_mcts.load(atomic::Ordering::Relaxed) {}
                        self.start_mcts_thread();
                    }
                } else {
                    self.current_input.input(key);
                }
            }
        }
        Ok(())
    }

    fn try_set_new_root(&mut self) -> bool {
        let line = &self.current_input.lines()[0];
        if let Ok(mov) = line.parse() {
            if let Some(new_node) = Node::find_child(self.root.clone(), mov) {
                self.root = new_node;
                return true;
            }
        }
        false
    }
}
impl<'a> Widget for &mut App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from("Ride the bus");

        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .horizontal_margin(2)
            .vertical_margin(1)
            .constraints(vec![Constraint::Percentage(100), Constraint::Length(3)])
            .split(area);

        let outer_block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let text_block = Block::bordered().title(Line::from("Enter move"));

        self.current_input.set_block(text_block);
        self.current_input.render(layout[1], buf);

        Paragraph::new(
            self.best_moves
                .iter()
                .map(|(m, x)| Line::from(format!("{:?} {:.3}", m, x)))
                .collect::<Vec<Line>>(),
        )
        .render(layout[0], buf);
        outer_block.render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let root = Node::start();

    let mut app = App::new(root);
    app.run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
