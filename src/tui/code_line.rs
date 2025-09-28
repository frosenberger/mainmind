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

impl Widget for CodeLine {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span::raw(" "));
        for c in self.code.iter() {
            let span = match c {
                Code::Red => Span::raw(format!("{}{}", c.glyph(), ' ')).red().bold(),
                Code::Green => Span::raw(format!("{}{}", c.glyph(), ' ')).green().bold(),
                Code::Yellow => Span::raw(format!("{}{}", c.glyph(), ' ')).yellow().bold(),
                Code::Blue => Span::raw(format!("{}{}", c.glyph(), ' ')).blue().bold(),
                Code::Magenta => Span::raw(format!("{}{}", c.glyph(), ' ')).magenta().bold(),
                Code::Cyan => Span::raw(format!("{}{}", c.glyph(), ' ')).cyan().bold(),
            };
            spans.push(span);
        }
        let line = Line::from(spans);
        line.render(area, buf);
    }
}
