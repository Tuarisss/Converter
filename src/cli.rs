use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long)] // аргумент короткий = -i длинный = -- imput 
    pub input: String,

    #[arg(short, long)]
    pub output: String,

    #[arg(short, long)]
    pub kind: MediaType,

    // #[arg(short, long)]
    // pub help: bool,

}

#[derive(ValueEnum, Clone)]
pub enum MediaType { // описываем тип который хочет пользователь только 2 возможных варианта
    Photo,
    Video,
}