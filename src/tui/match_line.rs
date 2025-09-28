use ratatui::{
    style::Stylize,
    text::{Line, Span},
    widgets::Widget,
};

use crate::code::Match;

#[derive(Default)]
pub struct MatchLine {
    pub matches: Vec<Match>,
}

impl From<Vec<Match>> for MatchLine {
    fn from(value: Vec<Match>) -> Self {
        MatchLine { matches: value }
    }
}

impl Widget for MatchLine {
    fn render(mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span::raw(" "));
        self.matches.sort();
        for m in self.matches.iter() {
            let span = match m {
                Match::No => Span::raw(format!("{}{}", m.glyph(), ' '))
                    .bold()
                    .gray()
                    .on_gray(),
                Match::Partial => Span::raw(format!("{}{}", m.glyph(), ' '))
                    .bold()
                    .white()
                    .on_gray(),
                Match::Full => Span::raw(format!("{}{}", m.glyph(), ' '))
                    .bold()
                    .black()
                    .on_gray(),
            };
            spans.push(span);
        }
        let line = Line::from(spans);
        line.render(area, buf);
    }
}
