use clap::*;
use colored::Colorize;

use crate::colors::Color;

mod colors;

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

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Command::Generate => print_code(gen_code(4)),
        Command::Play => println!("{}", "Not implemented yet".yellow()),
    }
}

fn gen_code(length: usize) -> Vec<Color> {
    let mut code = Vec::with_capacity(length);
    for _i in 0..length {
        code.push(Color::random());
    }
    code
}

fn print_code(code: Vec<Color>) {
    print!("[ ");
    for color in code {
        print!("{} ", color);
    }
    println!("]");
}
