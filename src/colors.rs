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

impl Clone for Color {
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

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Match {
    NO,
    PARTIAL,
    FULL,
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
            Match::NO => write!(f, " "),
            Match::PARTIAL => write!(f, "{}", "O".bold().white()),
            Match::FULL => write!(f, "{}", "X".bold().black().on_white()),
        }
    }
}
