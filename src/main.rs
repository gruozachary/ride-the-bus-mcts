mod card;
mod game;
mod node;

use std::{
    io,
    sync::{Arc, RwLock},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, read};
use node::Node;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use tui_textarea::TextArea;

struct App<'a> {
    root: Arc<RwLock<Node>>,
    current_input: TextArea<'a>,
    exit: bool,
}
impl<'a> App<'a> {
    fn new(root: Arc<RwLock<Node>>) -> App<'a> {
        App {
            root,
            current_input: TextArea::default(),
            exit: false,
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = read()? {
            if key.code == KeyCode::Esc {
                self.exit = true;
            } else {
                self.current_input.input(key);
            }
        }
        Ok(())
    }
}
impl<'a> Widget for &mut App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from("Ride the bus");

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        self.current_input.set_block(block);
        self.current_input.render(area, buf);
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
