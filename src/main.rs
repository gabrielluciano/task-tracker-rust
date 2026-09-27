mod task;

use clap::{Parser, Subcommand};

use crate::task::{TaskDatabase, TaskError};

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

fn add_handler(db: &mut impl TaskDatabase, title: &str) -> Result<(), TaskError> {
    let task = db.add(title)?;
    println!(
        "Task created: id='{}', title='{}', done='{}'",
        task.id(),
        task.title(),
        task.done()
    );
    Ok(())
}

fn list_handler(db: &impl TaskDatabase) -> Result<(), TaskError> {
    let tasks = db.list()?;
    if !tasks.is_empty() {
        println!("'id','title','done'");
    }
    for task in tasks {
        println!("'{}','{}',{}", task.id(), task.title(), task.done());
    }
    Ok(())
}

fn done_handler(db: &mut impl TaskDatabase, id: u32) -> Result<(), TaskError> {
    db.done(id)?;
    println!("Successfully changed status to done");
    Ok(())
}

fn remove_handler(db: &mut impl TaskDatabase, id: u32) -> Result<(), TaskError> {
    db.remove(id)?;
    println!("Successfully removed task.");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    // let mut db = task::memory_db::MemoryDb::new();
    let mut db = task::file_db::FileDb::new();

    // Use this when testing the MemoryDb implementation
    // test_memory_db(&mut db)?;

    match &cli.command {
        Commands::Add { title } => {
            add_handler(&mut db, title)?;
        }

        Commands::List {} => {
            list_handler(&db)?;
        }

        Commands::Done { id } => {
            done_handler(&mut db, *id)?;
        }

        Commands::Remove { id } => {
            remove_handler(&mut db, *id)?;
        }
    }
    Ok(())
}

fn test_memory_db(db: &mut impl TaskDatabase) -> Result<(), TaskError> {
    // Since this is in memory we need to do everything in single pass :(
    // so we manually call the handlers to test the behavior
    // Use task-tracker list to see results from below tests

    add_handler(db, "Task 1")?;
    add_handler(db, "Task 2")?;
    add_handler(db, "Task 3")?;
    list_handler(db)?;
    done_handler(db, 2)?;
    add_handler(db, "Task 4")?;
    remove_handler(db, 1)?;
    remove_handler(db, 4)?;

    // Error scenarios - first error will crash the program

    // uncomment other scenarios and comment the previous to test
    add_handler(db, "")?; // Empty title
    // remove_handler(db, 10)?; // Task doesn't exist
    // done_handler(db, 2)?; // Already done
    // done_handler(db, 10)?; // Task doesn't exist
    Ok(())
}
