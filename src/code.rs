use std::fmt::Display;

use rand::{seq::IteratorRandom, *};
use ratatui::style::Stylize;
use strum::*;

#[derive(Clone, Debug, EnumIter, PartialEq, PartialOrd)]
pub enum Code {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
}

impl Code {
    pub fn random() -> Code {
        Code::iter().choose(&mut rng()).unwrap()
    }

    pub fn glyph(&self) -> char {
        match &self {
            Code::Red => 'R',
            Code::Green => 'G',
            Code::Yellow => 'Y',
            Code::Blue => 'B',
            Code::Magenta => 'M',
            Code::Cyan => 'C',
        }
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Code::Red => write!(f, "{}", &self.glyph().to_string().bold().red()),
            Code::Green => write!(f, "{}", &self.glyph().to_string().bold().green()),
            Code::Yellow => write!(f, "{}", &self.glyph().to_string().bold().yellow()),
            Code::Blue => write!(f, "{}", &self.glyph().to_string().bold().blue()),
            Code::Magenta => write!(f, "{}", &self.glyph().to_string().bold().magenta()),
            Code::Cyan => write!(f, "{}", &self.glyph().to_string().bold().cyan()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Match {
    No,
    Partial,
    Full,
}

impl Match {
    pub fn glyph(&self) -> char {
        match &self {
            Match::No => ' ',
            Match::Partial => 'O',
            Match::Full => 'X',
        }
    }
}

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Match::No => write!(f, "{}", &self.glyph().to_string()),
            Match::Partial => write!(f, "{}", &self.glyph().to_string().bold().white().on_gray()),
            Match::Full => write!(f, "{}", &self.glyph().to_string().bold().black().on_gray()),
        }
    }
}
