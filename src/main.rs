mod task;

use clap::{Parser, Subcommand};

use crate::task::TaskDatabase;

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

fn add_handler(db: &mut impl TaskDatabase, title: &str) {
    db.add(title);
}

fn list_handler(db: &impl TaskDatabase) {
    let tasks = db.list();
    if !tasks.is_empty() {
        println!("'id','title','done'");
    }
    for task in tasks {
        println!("'{}','{}',{}", task.id(), task.title(), task.done());
    }
}

fn done_handler(db: &mut impl TaskDatabase, id: u32) {
    let updated = db.done(id);
    if updated {
        println!("Successfully changed status to done");
    } else {
        println!("Fail to set to done, task may already be done or not exist");
    }
}

fn remove_handler(db: &mut impl TaskDatabase, id: u32) {
    let removed = db.remove(id);
    if removed {
        println!("Successfully removed task.");
    } else {
        println!("Failed to remove task or it doesn't exist");
    }
}

fn main() {
    let cli = Cli::parse();
    let mut db = task::memory_db::MemoryDb::new();

    add_handler(&mut db, "Task 1");
    add_handler(&mut db, "Task 2");
    add_handler(&mut db, "Task 3");
    done_handler(&mut db, 2);
    add_handler(&mut db, "Task 4");
    remove_handler(&mut db, 1);
    remove_handler(&mut db, 4);

    match &cli.command {
        Commands::Add { title } => {
            add_handler(&mut db, title);
        }

        Commands::List {} => {
            list_handler(&db);
        }

        Commands::Done { id } => {
            done_handler(&mut db, *id);
        }

        Commands::Remove { id } => {
            remove_handler(&mut db, *id);
        }
    }
}
