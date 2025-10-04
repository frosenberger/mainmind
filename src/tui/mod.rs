use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::*,
    style::Stylize,
    text::ToLine,
    widgets::{Block, BorderType, Widget},
};

use crate::{
    code::Code,
    engine::Game,
    tui::{code_line::CodeLine, result_list::ResultList},
};

mod code_line;
mod guess_line;
mod match_line;
mod result_list;

#[derive(Debug, Default)]
pub struct App {
    debug: bool,
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
            KeyCode::Enter => {
                self.game.add_guess(&self.input);
                self.input.clear();
            }
            _ => {}
        }
    }

    fn add_code(&mut self, code: Code) {
        if self.game.code_length > (self.input.len() as u8) {
            self.input.push(code);
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let app_layout = AppLayout::of_game(&self.game, area, buf);

        let result_list = ResultList::of_game(&self.game);
        result_list.render(app_layout.top, buf);

        let code_input = CodeLine {
            code: self.input.clone(),
        };
        code_input.render(app_layout.bot, buf);

        if self.game.won {
            "You win!"
                .to_line()
                .centered()
                .italic()
                .slow_blink()
                .render(app_layout.status, buf);
        } else if self.debug {
            let code = self.game.code.clone();
            let status = format!("(solution: {:?})", code);
            status.to_line().render(app_layout.status, buf);
        }
    }
}

pub fn run(debug: bool) -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    app.debug = debug;
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}

struct AppLayout {
    top: Rect,
    bot: Rect,
    status: Rect,
}

impl AppLayout {
    fn of_game(game: &Game, area: Rect, buf: &mut Buffer) -> AppLayout {
        let full_height = Layout::vertical([Constraint::Percentage(100)])
            .flex(Flex::Center)
            .split(area);
        let full_width = Layout::horizontal([Constraint::Percentage(100)])
            .flex(Flex::Center)
            .split(full_height[0]);
        let vert_split =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).split(full_width[0]);
        let center = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(26),
            Constraint::Fill(1),
        ])
        .split(vert_split[0]);
        let layout = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length((game.max_rounds * 3 + 2).into()),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(center[1]);

        let top_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(" Mainmind ".bold())
            .title_alignment(Alignment::Center);
        let top = top_block.inner(layout[1]);
        top_block.render(layout[1], buf);

        let bot_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Input");
        let bot = bot_block.inner(layout[2]);
        bot_block.render(layout[2], buf);

        let status_block = Block::bordered()
            .border_type(BorderType::Plain)
            .title_position(ratatui::widgets::block::Position::Top)
            .title_alignment(Alignment::Center)
            .title("Status");
        let status = status_block.inner(vert_split[1]);
        status_block.render(vert_split[1], buf);

        AppLayout { top, bot, status }
    }
}
