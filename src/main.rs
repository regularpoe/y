use chrono::{Local, NaiveDate};
use clap::{Parser, Subcommand};
use dirs::home_dir;
use rusqlite::{Connection, OptionalExtension, Result};

use std::fmt;

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
    ViewById {
        id: i32,
    },
    Edit {
        id: i32,
        date: Option<String>,
        description: Option<String>,
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

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Record ID: {}\nDate: {}\nDescription: {}",
            self.id, self.date, self.description
        )
    }
}

fn init_db() -> Result<Connection> {
    let home = home_dir().expect("Couldn't determine home directory");
    let db_path = home.join("y.db");
    let conn = Connection::open(&db_path)?;
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

    conn.execute(
        "INSERT INTO logs (date, description) VALUES (?1, ?2)",
        [&formatted_date, &description.to_string()],
    )
}

fn view_all(conn: &Connection) -> Result<Vec<Log>> {
    let mut stmt = conn.prepare("SELECT id, date, description FROM logs")?;
    let log_iter = stmt.query_map([], |row| {
        Ok(Log {
            id: row.get::<_, i32>(0)?,
            date: row.get::<_, String>(1)?.parse().unwrap(),
            description: row.get::<_, String>(2)?,
        })
    })?;

    let logs: Vec<Log> = log_iter.map(|log| log.unwrap()).collect();
    Ok(logs)
}

fn view_logs_by_date(conn: &Connection, date: NaiveDate) -> Result<Vec<Log>> {
    let mut stmt = conn.prepare("SELECT id, date, description FROM logs WHERE date = ?1")?;
    let log_iter = stmt.query_map([date.to_string()], |row| {
        Ok(Log {
            id: row.get::<_, i32>(0)?,
            date: row.get::<_, String>(1)?.parse().unwrap(),
            description: row.get::<_, String>(2)?,
        })
    })?;

    let logs: Vec<Log> = log_iter.map(|log| log.unwrap()).collect();
    Ok(logs)
}

fn view_by_id(conn: &Connection, id: i32) -> Result<Option<Log>> {
    let mut stmt = conn.prepare("SELECT id, date, description FROM logs WHERE id = ?1")?;

    let log = stmt
        .query_row([id], |row| {
            Ok(Log {
                id: row.get::<_, i32>(0)?,
                date: row.get::<_, String>(1)?.parse().unwrap(),
                description: row.get::<_, String>(2)?,
            })
        })
        .optional()?;

    Ok(log)
}

fn edit(
    conn: &Connection,
    id: i32,
    date: Option<NaiveDate>,
    description: Option<&str>,
) -> Result<usize> {
    let mut query = String::from("UPDATE logs SET ");
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(date) = date {
        let formatted_date = date.format("%Y-%m-%d").to_string();
        query.push_str("date = ?1");
        params.push(Box::new(formatted_date));
    }

    if let Some(description) = description {
        if !params.is_empty() {
            query.push_str(", ");
        }
        query.push_str("description = ?2");
        params.push(Box::new(description.to_string()));
    }

    query.push_str(" WHERE id = ?3");
    params.push(Box::new(id));

    let mut stmt = conn.prepare(&query)?;

    let params: Vec<&(dyn rusqlite::ToSql + 'static)> = params.iter().map(|p| p.as_ref()).collect();

    stmt.execute(params.as_slice())
}

fn delete(conn: &Connection, id: i32) -> Result<usize> {
    conn.execute("DELETE FROM logs WHERE id = ?1", [&id])
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

            add(&conn, date, &description)?;
            println!("Record added!");
        }
        Commands::ViewAll => {
            let records = view_all(&conn)?;
            for record in records {
                println!("\n{}\n", record);
            }
        }
        Commands::View { date } => {
            let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
            let records = view_logs_by_date(&conn, date)?;
            for record in records {
                println!("\n{}\n", record);
            }
        }
        Commands::ViewById { id } => match view_by_id(&conn, id)? {
            Some(log) => println!("{}", log),
            None => println!("No records found with ID: {}", id),
        },
        Commands::Edit {
            id,
            date,
            description,
        } => {
            let date = date
                .as_deref()
                .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d"))
                .transpose()?;

            edit(&conn, id, date, description.as_deref())?;
            println!("Record updated!");

            match view_by_id(&conn, id) {
                Ok(log) => println!("{}", log.unwrap()),
                Err(_) => println!("Cannot fetch record with ID {}", id),
            }
        }
        Commands::Delete { id } => {
            delete(&conn, id)?;
            println!("Record deleted!");
        }
    }

    Ok(())
}
