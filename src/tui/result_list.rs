use ratatui::{
    layout::{Constraint, Layout},
    widgets::Widget,
};

use crate::{
    engine::Game,
    tui::{code_line::CodeLine, guess_line::GuessLine, match_line::MatchLine},
};

pub struct ResultList {
    pub num_items: u8,
    pub guess_lines: Vec<GuessLine>,
}

impl ResultList {
    pub fn of_game(game: &Game) -> ResultList {
        let mut lines: Vec<GuessLine> = Vec::new();
        for i in 0..game.round {
            lines.push(GuessLine {
                code_line: CodeLine {
                    code: game.guesses.iter().nth(i.into()).unwrap_or(&vec![]).clone(),
                },
                match_line: MatchLine {
                    matches: game.matches.iter().nth(i.into()).unwrap_or(&vec![]).clone(),
                },
                number: i + 1,
            });
        }
        for i in game.round..game.max_rounds {
            lines.push(GuessLine::empty(i + 1));
        }
        ResultList {
            num_items: game.max_rounds,
            guess_lines: lines.into_iter().rev().collect(),
        }
    }
}

impl Widget for ResultList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut constraints: Vec<Constraint> = Vec::new();
        for _n in 0..self.num_items {
            constraints.push(Constraint::Length(3));
        }
        let layout = Layout::vertical(constraints).split(area);
        self.guess_lines
            .into_iter()
            .enumerate()
            .rev()
            .for_each(|line| line.1.render(layout[line.0], buf));
    }
}
