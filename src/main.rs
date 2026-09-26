use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        // Default struct fields (no macro) are positional arguments
        title: String,
    },
    List {},
    Done {
        id: u32,
    },
    Remove {
        id: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { title } => {
            println!("Adding task with title: {title}");
        }
        Commands::List {} => {
            println!("Listing all tasks");
        }
        Commands::Done { id } => {
            println!("Marking id {id} as done");
        }
        Commands::Remove { id } => {
            println!("Removing task with id {id}");
        }
    }
}
