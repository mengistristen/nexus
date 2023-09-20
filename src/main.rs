use crate::args::{Args, Commands, Parser};

pub mod args;

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Create { file_name } => println!("file_name: {file_name}"),
    };
}
