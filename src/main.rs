mod photo;
mod video;
mod cli;
mod core;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    // получаем параметры от пользователя и передаём в core
    core::convert(
        &cli.kind,
        &cli.output,
        &cli.input,
    )?;
    Ok(())
}