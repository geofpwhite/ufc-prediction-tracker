use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use tracing::event;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub fn get_db_connection() -> Result<Connection> {
    let path = "./database.db";
    Connection::open(path)
}

pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            date DATE NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS results (
            event_id INTEGER NOT NULL,
            winner TEXT NOT NULL,
            loser TEXT NOT NULL,
            unique (event_id, winner, loser)
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS predictions (
            event_id INTEGER NOT NULL,
            winner TEXT NOT NULL,
            loser TEXT NOT NULL,
            unique (event_id, winner, loser)
        )",
        (),
    )?;
    Ok(())
}

pub struct ResultRow {
    event_id: i32,
    winner: String,
    loser: String,
    method: String,
    round: i32,
}

pub fn add_result(conn: &Connection, result: ResultRow) -> Result<usize> {
    let sql =
        "INSERT INTO results (event_id, winner, loser, method, round) VALUES (?1, ?2, ?3, ?4, ?5)";
    let params = (result.event_id, result.winner, result.loser);
    conn.execute(sql, params)
}

pub fn add_event(conn: &Connection, name: &str, date: &str) -> Result<usize> {
    match conn.query_row(
        "SELECT id FROM events WHERE name=?1 and date=?2",
        (name, date),
        |row| row.get(0),
    ) {
        Ok(id) => Ok(id),
        Err(_) => {
            conn.execute(
                "INSERT OR IGNORE INTO events (name, date) VALUES (?1, ?2)",
                (name, date),
            )
            .unwrap();
            conn.query_row(
                "SELECT id FROM events WHERE name=?1 and date=?2",
                (name, date),
                |row| row.get(0),
            )
        }
    }
}

pub fn add_or_update_prediction(
    conn: &Connection,
    event_id: usize,
    winner: &str,
    loser: &str,
) -> Result<usize> {
    match conn.query_row(
        "SELECT event_id FROM predictions WHERE event_id=?1 AND winner=?3 AND loser=?2",
        (event_id, winner, loser),
        |row: &rusqlite::Row<'_>| row.get::<_, usize>(0),
    ) {
        Ok(rowid) => conn.execute(
            "UPDATE predictions set (event_id, winner, loser) = (?1, ?2, ?3) where winner=?3 and loser=?2",
            (event_id, winner, loser),
        ),
        Err(_) => conn.execute(
            "INSERT INTO predictions (event_id, winner, loser) VALUES (?1, ?2, ?3)",
            (event_id, winner, loser),
        ),
    }
}

pub fn get_predictions(conn: &Connection, event_id: usize) -> Result<Vec<(String, String)>> {
    let mut statement = conn.prepare("SELECT winner,loser FROM predictions WHERE event_id=?1")?;
    let mut rows = statement.query((event_id,))?;
    let mut predictions: Vec<(String, String)> = vec![];
    while let Some(row) = rows.next()? {
        let winner: String = row.get(0)?;
        let loser: String = row.get(1)?;
        predictions.push((winner.clone(), loser.clone()));
        println!("{winner} {loser}");
    }
    Ok(predictions)
}
