use crate::code::Code;
use ratatui::{
    style::Stylize,
    text::{Line, Span},
    widgets::Widget,
};

#[derive(Default)]
pub struct Code_Line {
    pub code: Vec<Code>,
}

impl Widget for Code_Line {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span::raw(" "));
        for c in self.code.iter() {
            let span = match c {
                Code::RED => Span::raw(format!("{}{}", c.glyph(), ' ')).red().bold(),
                Code::GREEN => Span::raw(format!("{}{}", c.glyph(), ' ')).green().bold(),
                Code::YELLOW => Span::raw(format!("{}{}", c.glyph(), ' ')).yellow().bold(),
                Code::BLUE => Span::raw(format!("{}{}", c.glyph(), ' ')).blue().bold(),
                Code::MAGENTA => Span::raw(format!("{}{}", c.glyph(), ' ')).magenta().bold(),
                Code::CYAN => Span::raw(format!("{}{}", c.glyph(), ' ')).cyan().bold(),
            };
            spans.push(span);
        }
        let line = Line::from(spans);
        line.render(area, buf);
    }
}
