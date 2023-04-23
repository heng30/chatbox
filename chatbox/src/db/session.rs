use crate::config;
use rusqlite::{Connection, Result};

fn connection() -> Result<Connection> {
    let (_, _, db_path) = config::path();
    Connection::open(db_path)
}

pub fn init() -> Result<()> {
    let conn = connection()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
             id INTEGER PRIMARY KEY,
             uuid TEXT NOT NULL UNIQUE,
             config TEXT NOT NULL,
             chats TEXT
             )",
        [],
    )?;

    Ok(())
}

pub fn delete(uuid: String) -> Result<()> {
    let conn = connection()?;
    conn.execute("DELETE FROM sessions WHERE uuid=?", [uuid])?;
    Ok(())
}

pub fn insert(uuid: String, config: String, chats: String) -> Result<()> {
    let conn = connection()?;

    conn.execute(
        "INSERT INTO sessions (uuid, config, chats) VALUES (?, ?, ?)",
        [uuid, config, chats],
    )?;
    Ok(())
}

pub fn update(uuid: String, config: Option<String>, chats: Option<String>) -> Result<()> {
    let conn = connection()?;

    if config.is_some() && chats.is_some() {
        conn.execute(
            "UPDATE sessions SET config=?, chats=?  WHERE uuid=?",
            [config.unwrap(), chats.unwrap(), uuid],
        )?;
    } else if config.is_some() {
        conn.execute(
            "UPDATE sessions SET config=? WHERE uuid=?",
            [config.unwrap(), uuid],
        )?;
    } else if chats.is_some() {
        conn.execute(
            "UPDATE sessions SET chats=?  WHERE uuid=?",
            [chats.unwrap(), uuid],
        )?;
    }

    Ok(())
}

#[allow(dead_code)]
pub fn select(uuid: String) -> Result<Option<(String, String)>> {
    let conn = connection()?;

    let mut stmt = conn.prepare(&format!(
        "SELECT config, chats FROM sessions WHERE uuid='{}'",
        uuid
    ))?;
    let mut rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;

    if let Some(Ok(row)) = rows.next() {
        return Ok(Some((row.0, row.1)));
    }
    Ok(None)
}

pub fn select_all() -> Result<Vec<(String, String, String)>> {
    let conn = connection()?;

    let mut stmt = conn.prepare("SELECT uuid, config, chats FROM sessions")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    Ok(rows.flatten().collect())
}

pub fn is_exist(uuid: &str) -> Result<bool> {
    let conn = connection()?;
    let cnt = conn.query_row::<i64, _, _>(
        &format!("SELECT 1 FROM sessions WHERE uuid='{}'", uuid),
        [],
        |r| r.get(0),
    )?;
    Ok(cnt == 1)
}
