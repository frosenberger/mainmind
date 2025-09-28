use crate::code::Code;
use ratatui::{
    style::Stylize,
    text::{Line, Span},
    widgets::Widget,
};

#[derive(Default)]
pub struct CodeLine {
    pub code: Vec<Code>,
}

impl From<Vec<Code>> for CodeLine {
    fn from(value: Vec<Code>) -> Self {
        CodeLine { code: value }
    }
}

impl Widget for CodeLine {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span::raw(" "));
        for c in self.code.iter() {
            let span = match c {
                Code::Red => Span::raw(format!("{}{}", c.glyph(), ' ')).bold().red(),
                Code::Green => Span::raw(format!("{}{}", c.glyph(), ' ')).bold().green(),
                Code::Yellow => Span::raw(format!("{}{}", c.glyph(), ' ')).bold().yellow(),
                Code::Blue => Span::raw(format!("{}{}", c.glyph(), ' ')).bold().blue(),
                Code::Magenta => Span::raw(format!("{}{}", c.glyph(), ' ')).bold().magenta(),
                Code::Cyan => Span::raw(format!("{}{}", c.glyph(), ' ')).bold().cyan(),
            };
            spans.push(span);
        }
        let line = Line::from(spans);
        line.render(area, buf);
    }
}
