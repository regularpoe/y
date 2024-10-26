use chrono::{Local, NaiveDate};
use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

#[derive(Parser)]
#[command(name = "y")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        description: String,
        date: Option<String>,
    },
    ViewAll,
    View {
        date: String,
    },
    Edit {
        id: i32,
        description: String,
    },
    Delete {
        id: i32,
    },
}

#[derive(Debug)]
struct Log {
    id: i32,
    date: NaiveDate,
    description: String,
}

fn init_db() -> Result<Connection> {
    let conn = Connection::open("y.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            description TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

// CRUD

fn add(conn: &Connection, date: Option<NaiveDate>, description: &str) -> Result<usize> {
    let date = date.unwrap_or_else(|| Local::now().naive_local().into());
    let formatted_date = date.format("%Y-%m-%d").to_string();

    println!("Today is {}", formatted_date);

    Ok(1)
}

fn view_all() {
    println!("View all called");
}

fn view_by_date() {
    println!("View by date called");
}

fn edit() {
    println!("Edit called");
}

fn delete() {
    println!("Delete called");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let conn = init_db()?;

    match cli.command {
        Commands::Add { description, date } => {
            let date = date
                .as_deref()
                .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d"))
                .transpose()?;

            add(&conn, date, &description);
        }
        Commands::ViewAll => {
            view_all();
        }
        Commands::View { date } => {
            view_by_date();
        }
        Commands::Edit { id, description } => {
            edit();
        }
        Commands::Delete { id } => {
            delete();
        }
    }

    Ok(())
}
