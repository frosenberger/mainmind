use std::io;

use clap::*;

use crate::code::Code;

mod code;
mod engine;
mod tui;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(short, long)]
    debug: bool,
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
        Command::Play => tui::run(cli.debug),
    }
}

fn print_code(code: Vec<Code>) -> io::Result<()> {
    print!("[ ");
    for color in code {
        print!("{} ", color);
    }
    println!("]");
    Ok(())
}
