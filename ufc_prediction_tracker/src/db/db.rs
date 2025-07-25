use rusqlite::{Connection, Map, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tracing::event;

pub type SharedConnection = Arc<Mutex<Connection>>;

pub fn create_shared_connection(path: &str) -> SharedConnection {
    Arc::new(Mutex::new(
        Connection::open(path).expect("Failed to open DB"),
    ))
}

#[derive(Clone)]
pub struct Store {
    pub conn: SharedConnection,
}

impl Store {
    pub fn new(path: &str) -> Self {
        Store {
            conn: create_shared_connection(path),
        }
    }
    pub fn create_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            date DATE NOT NULL,
            link TEXT NOT NULL,
            unique (name,date)
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
    pub fn add_or_update_result(
        &self,
        event_id: usize,
        winner: &str,
        loser: &str,
    ) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        match conn.query_row(
            "SELECT event_id FROM results WHERE event_id=?1 AND winner=?3 AND loser=?2",
            (event_id, winner, loser),
            |row: &rusqlite::Row<'_>| row.get::<_, usize>(0),
        ) {
            Ok(_) => conn.execute(
                "UPDATE results set (event_id, winner, loser) = (?1, ?2, ?3) where winner=?3 and loser=?2",
                (event_id, winner, loser),
            ),
            Err(_) => conn.execute(
                "INSERT INTO results (event_id, winner, loser) VALUES (?1, ?2, ?3)",
                (event_id, winner, loser),
            ),
        }
    }

    pub fn add_event(&self, name: &str, date: &str, link: &str) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        match conn.query_row(
            "SELECT id FROM events WHERE name=?1 and date=?2",
            (name, date),
            |row| row.get(0),
        ) {
            Ok(id) => Ok(id),
            Err(_) => {
                conn.execute(
                    "INSERT OR IGNORE INTO events (name, date, link) VALUES (?1, ?2,?3)",
                    (name, date, link),
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
        &self,
        event_id: usize,
        winner: &str,
        loser: &str,
    ) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        match conn.query_row(
            "SELECT event_id FROM predictions WHERE event_id=?1 AND winner=?3 AND loser=?2",
            (event_id, winner, loser),
            |row: &rusqlite::Row<'_>| row.get::<_, usize>(0),
        ) {
            Ok(_) => conn.execute(
                "UPDATE predictions set (event_id, winner, loser) = (?1, ?2, ?3) where winner=?3 and loser=?2",
                (event_id, winner, loser),
            ),
            Err(_) => conn.execute(
                "INSERT INTO predictions (event_id, winner, loser) VALUES (?1, ?2, ?3)",
                (event_id, winner, loser),
            ),
        }
    }

    pub fn get_predictions(&self, event_id: usize) -> Result<Vec<(String, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut statement: rusqlite::Statement<'_> =
            conn.prepare("SELECT winner,loser FROM predictions WHERE event_id=?1")?;
        let mut rows: rusqlite::Rows<'_> = statement.query((event_id,))?;
        let mut predictions: Vec<(String, String)> = vec![];
        while let Some(row) = rows.next()? {
            let winner: String = row.get(0)?;
            let loser: String = row.get(1)?;
            predictions.push((winner.clone(), loser.clone()));
            println!("{winner} {loser}");
        }
        Ok(predictions)
    }

    //returns correct,incorrect number of guesses
    pub fn get_my_predictions_correctness(&self) -> Result<(i64, i64)> {
        let conn = self.conn.lock().unwrap();
        let correct: i64 = conn.query_row(
            "SELECT count(*) FROM results as r JOIN predictions as p ON p.event_id=r.event_id and p.winner=r.winner and p.loser=r.loser",
            (),
            |row: &rusqlite::Row<'_>| row.get(0),
        )?;
        let incorrect: i64 = conn.query_row(
            "SELECT count(*) FROM results as r JOIN predictions as p ON p.event_id=r.event_id and p.loser=r.winner and p.winner=r.loser",
            (),
            |row: &rusqlite::Row<'_>| row.get(0),
        )?;

        Ok((correct, incorrect))
    }
    pub fn get_my_predictions_correctness_for_event(&self, id: usize) -> Result<(i64, i64)> {
        let conn = self.conn.lock().unwrap();
        let correct: i64 = conn.query_row(
            "SELECT count(*) FROM results as r JOIN predictions as p ON p.event_id=r.event_id and p.winner=r.winner and p.loser=r.loser where p.event_id=?1",
            (id,),
            |row: &rusqlite::Row<'_>| row.get(0),
        )?;
        let incorrect: i64 = conn.query_row(
            "SELECT count(*) FROM results as r JOIN predictions as p ON p.event_id=r.event_id and p.loser=r.winner and p.winner=r.loser",
            (),
            |row: &rusqlite::Row<'_>| row.get(0),
        )?;

        Ok((correct, incorrect))
    }

    pub fn get_past_events_with_predictions(&self) -> Result<Vec<(usize, String, String, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt: rusqlite::Statement<'_> = conn.prepare("SELECT DISTINCT events.id, events.name, events.date,link FROM events where events.date < date('now')").unwrap();
        let rows = stmt
            .query_map([], |row| {
                let id: usize = row.get(0)?;
                let name: String = row.get(1)?;
                let date: String = row.get(2)?;
                let link: String = row.get(3)?;
                Ok((id, name, date, link))
            })
            .unwrap();
        let mut result: Vec<(usize, String, String, String)> = Vec::new();
        for row in rows {
            result.push(row.unwrap());
        }
        Ok(result)
    }
}
