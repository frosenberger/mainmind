use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{self, Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Paragraph, Widget},
};

use crate::engine::Game;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    game: Game,
    input: String,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('r') => self.input.push('R'),
            KeyCode::Char('g') => self.input.push('G'),
            KeyCode::Char('y') => self.input.push('Y'),
            KeyCode::Char('b') => self.input.push('B'),
            KeyCode::Char('m') => self.input.push('M'),
            KeyCode::Char('c') => self.input.push('C'),
            KeyCode::Backspace => {
                if self.input.len() > 0 {
                    self.input.truncate(self.input.len() - 1)
                }
            }
            KeyCode::Enter => {} // TODO confirm guess
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Constraint::Length(9), Constraint::Length(3)]).split(area);
        let top = Block::bordered()
            .title(" Mainmind ".bold())
            .title_alignment(layout::Alignment::Center);
        let bot = Block::bordered();
        let input = Paragraph::new(self.input.clone()).block(bot);
        top.render(layout[0], buf);
        input.render(layout[1], buf);
    }
}

pub fn run() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
