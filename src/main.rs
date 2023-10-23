use args::{Args, Commands, Parser};
use nexus_core::{
    commands::{create::create_note, link::link_note},
    utils::get_data_dir,
};

pub mod args;

fn main() {
    let args = Args::parse();
    let data_dir = get_data_dir();

    let result = match args.command {
        Commands::Create { name } => create_note(data_dir, name),
        Commands::Link {
            source,
            target,
            branch,
        } => link_note(data_dir, source, target, branch),
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
