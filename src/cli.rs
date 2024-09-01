use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "expense-tracker")]
#[command(version = "1.0")]
#[command(about = "A simple expense tracker CLI tool")]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Add {
        #[clap(short, long)]
        description: String,
        #[clap(short, long)]
        amount: f32,
    },
    List,
    Summary {
        #[clap(short, long)]
        month: Option<u8>,
    },
    Delete {
        #[clap(short, long)]
        id: uuid::Uuid,
    },
}
