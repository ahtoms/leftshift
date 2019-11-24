use rusqlite::{Connection, OpenFlags, Result, params};
use crate::db::event::Event;

macro_rules! DB_DEFAULTS {
    (DB_PATH) => ("./lshift.db");
    (DB_EVENT_LIMIT) => ("SELECT id, title, description, location, tstamp, pub_tstamp, img_loc FROM events LIMIT (?1);")
}

pub fn establish() -> Result<Connection> {
    Connection::open_with_flags(DB_DEFAULTS!(DB_PATH), OpenFlags::SQLITE_OPEN_READ_ONLY)
}

pub fn get_events(conn: &Connection, limit: u32) -> Vec<Event> {
    let stmt_events = conn.prepare(DB_DEFAULTS!(DB_EVENT_LIMIT));
    match stmt_events {
        Ok(mut stmt) => {
            if let Ok(events) = stmt.query_map(params!(limit), |row| {
                Ok(
                    Event::new(row.get(0)?, 
                        row.get(1)?, row.get(2)?, 
                        row.get(3)?, row.get(6)?, 
                        row.get(4)?, row.get(5)?)
                ) 
            }) {
                return events.filter_map(Result::ok).collect()
            }
            Vec::new()
        },
        Err(e) => { eprintln!("{}", e); Vec::new() }
    } 
}

