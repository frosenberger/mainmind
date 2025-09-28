use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, BorderType, Widget},
};

use crate::tui::{code_line::CodeLine, match_line::MatchLine};

pub struct GuessLine {
    pub number: u8,
    pub code_line: CodeLine,
    pub match_line: MatchLine,
}

impl GuessLine {
    pub fn empty(number: u8) -> GuessLine {
        GuessLine {
            number,
            code_line: CodeLine { code: vec![] },
            match_line: MatchLine { matches: vec![] },
        }
    }
}

impl Widget for GuessLine {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let height = Layout::vertical([Constraint::Length(3)]).split(area);
        let layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(height[0]);
        let code_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(format!("({})", self.number));
        let code_block_content = code_block.inner(layout[0]);
        let match_block = Block::bordered().border_type(BorderType::Rounded);
        let match_block_content = match_block.inner(layout[1]);

        code_block.render(layout[0], buf);
        match_block.render(layout[1], buf);
        self.code_line.render(code_block_content, buf);
        self.match_line.render(match_block_content, buf);
    }
}
