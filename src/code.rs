use std::fmt::Display;

use colored::Colorize;
use rand::{seq::IteratorRandom, *};
use strum::*;

#[derive(EnumIter, Debug, PartialEq, PartialOrd)]
pub enum Code {
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
}

impl Code {
    pub fn random() -> Code {
        Code::iter().choose(&mut rng()).unwrap()
    }

    pub fn glyph(&self) -> char {
        match &self {
            Code::RED => 'R',
            Code::GREEN => 'G',
            Code::YELLOW => 'Y',
            Code::BLUE => 'B',
            Code::MAGENTA => 'M',
            Code::CYAN => 'C',
        }
    }
}

impl Clone for Code {
    fn clone(&self) -> Self {
        match self {
            Self::RED => Self::RED,
            Self::GREEN => Self::GREEN,
            Self::YELLOW => Self::YELLOW,
            Self::BLUE => Self::BLUE,
            Self::MAGENTA => Self::MAGENTA,
            Self::CYAN => Self::CYAN,
        }
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Code::RED => write!(f, "{}", &self.glyph().to_string().bold().red()),
            Code::GREEN => write!(f, "{}", &self.glyph().to_string().bold().green()),
            Code::YELLOW => write!(f, "{}", &self.glyph().to_string().bold().yellow()),
            Code::BLUE => write!(f, "{}", &self.glyph().to_string().bold().blue()),
            Code::MAGENTA => write!(f, "{}", &self.glyph().to_string().bold().magenta()),
            Code::CYAN => write!(f, "{}", &self.glyph().to_string().bold().cyan()),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Match {
    NO,
    PARTIAL,
    FULL,
}

impl Match {
    pub fn glyph(&self) -> char {
        match &self {
            Match::NO => ' ',
            Match::PARTIAL => 'O',
            Match::FULL => 'X',
        }
    }
}

impl Clone for Match {
    fn clone(&self) -> Self {
        match self {
            Self::NO => Self::NO,
            Self::PARTIAL => Self::PARTIAL,
            Self::FULL => Self::FULL,
        }
    }
}

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Match::NO => write!(f, "{}", &self.glyph().to_string()),
            Match::PARTIAL => write!(f, "{}", &self.glyph().to_string().bold().white()),
            Match::FULL => write!(f, "{}", &self.glyph().to_string().bold().black().on_white()),
        }
    }
}
