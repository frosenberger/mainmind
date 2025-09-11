use std::fmt::Display;

use colored::Colorize;
use rand::{seq::IteratorRandom, *};
use strum::*;

#[derive(EnumIter, Debug, PartialEq, PartialOrd)]
pub enum Color {
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
}

impl Color {
    pub fn random() -> Color {
        Color::iter().choose(&mut rng()).unwrap()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Color::RED => write!(f, "{}", "R".bold().red()),
            Color::GREEN => write!(f, "{}", "G".bold().green()),
            Color::YELLOW => write!(f, "{}", "Y".bold().yellow()),
            Color::BLUE => write!(f, "{}", "B".bold().blue()),
            Color::MAGENTA => write!(f, "{}", "M".bold().magenta()),
            Color::CYAN => write!(f, "{}", "C".bold().cyan()),
        }
    }
}
