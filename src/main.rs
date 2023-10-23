use args::{Args, Parser, Commands};
use nexus_core::{utils::get_data_dir, commands::create::create_note};

pub mod args;

fn main() {
    let args = Args::parse();
    let data_dir = get_data_dir(); 

    match args.command {
        Commands::Create { name } => create_note(data_dir, name),
    };
}
