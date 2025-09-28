use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{self, Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Widget},
};

use crate::{code::Code, engine::Game, tui::code_line::CodeLine};

mod code_line;
mod match_line;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    game: Game,
    input: Vec<Code>,
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
            KeyCode::Char('q') | KeyCode::Char('Q') => self.exit(),
            KeyCode::Char('r') | KeyCode::Char('R') => self.add_code(Code::Red),
            KeyCode::Char('g') | KeyCode::Char('G') => self.add_code(Code::Green),
            KeyCode::Char('y') | KeyCode::Char('Y') => self.add_code(Code::Yellow),
            KeyCode::Char('b') | KeyCode::Char('B') => self.add_code(Code::Blue),
            KeyCode::Char('m') | KeyCode::Char('M') => self.add_code(Code::Magenta),
            KeyCode::Char('c') | KeyCode::Char('C') => self.add_code(Code::Cyan),
            KeyCode::Backspace => {
                if self.input.len() > 0 {
                    self.input.pop();
                }
            }
            KeyCode::Enter => {} // TODO confirm guess
            _ => {}
        }
    }

    fn add_code(&mut self, code: Code) {
        if self.game.code_length > self.input.len() {
            self.input.push(code);
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
        let bot = Block::bordered().title("Input");
        let bot_inner = bot.inner(layout[1]);
        let code_input = CodeLine {
            code: self.input.clone(),
        };
        top.render(layout[0], buf);
        bot.render(layout[1], buf);
        code_input.render(bot_inner, buf);
    }
}

pub fn run() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
