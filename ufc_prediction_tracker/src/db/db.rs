use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

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
            method TEXT NOT NULL,
            round INTEGER NOT NULL,
            unique (event_id, winner, loser)
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS predictions (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            winner TEXT NOT NULL,
            loser TEXT NOT NULL,
            method TEXT NOT NULL,
            round INTEGER NOT NULL
        )",
        (),
    )?;
    Ok(())
}

pub fn create_prediction(
    conn: &Connection,
    name: &str,
    winner: &str,
    loser: &str,
    method: &str,
    round: i32,
) -> Result<()> {
    conn.execute(
        "INSERT INTO predictions (name, winner, loser, method, round) VALUES (?1, ?2, ?3, ?4, ?5)",
        (name, winner, loser, method, round),
    )?;
    Ok(())
}

pub struct ResultRow {
    id: i32,
    name: String,
    winner: String,
    loser: String,
    method: String,
    round: i32,
}

pub fn add_result(conn: &Connection, result: ResultRow) -> Result<usize> {
    // This function would contain the logic to scrape the latest
    let sql =
        "INSERT INTO results (name, winner, loser, method, round) VALUES (?1, ?2, ?3, ?4, ?5)";
    let params = (
        result.name,
        result.winner,
        result.loser,
        result.method,
        result.round,
    );
    conn.execute(sql, params)
}

pub fn add_event(conn: &Connection, name: &str, date: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO events (name, date) VALUES (?1, ?2)",
        (name, date),
    )?;
    Ok(())
}

pub fn add_prediction(
    conn: &Connection,
    name: &str,
    winner: &str,
    loser: &str,
    method: &str,
    round: i32,
) -> Result<()> {
    conn.execute(
        "INSERT INTO predictions (name, winner, loser, method, round) VALUES (?1, ?2, ?3, ?4, ?5)",
        (name, winner, loser, method, round),
    )?;
    Ok(())
}
