use chrono::NaiveDate;
use rusqlite::{Connection, Result};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = init_db()?;

    Ok(())
}
