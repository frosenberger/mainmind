use std::io;

use clap::*;

use crate::colors::Color;

mod colors;
mod engine;
mod tui;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Generate,
    Play,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Command::Generate => print_code(engine::gen_code(4)),
        Command::Play => tui::run(),
    }
}

fn print_code(code: Vec<Color>) -> io::Result<()> {
    print!("[ ");
    for color in code {
        print!("{} ", color);
    }
    println!("]");
    Ok(())
}
