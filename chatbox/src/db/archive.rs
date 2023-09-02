use crate::config;
use rusqlite::{params, Connection, Result};

fn connection() -> Result<Connection> {
    let (_, _, db_path) = config::path();
    Connection::open(db_path)
}

fn table_name(suuid: &str) -> String {
    "archive_".to_string() + &suuid.replace('-', "_")
}

pub fn new(suuid: &str) -> Result<()> {
    let conn = connection()?;

    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
             id INTEGER PRIMARY KEY,
             uuid TEXT NOT NULL UNIQUE,
             name TEXT NOT NULL,
             data TEXT NOT NULL
             )",
            table_name(suuid)
        ),
        [],
    )?;

    Ok(())
}

pub fn delete(suuid: &str, uuid: &str) -> Result<()> {
    let conn = connection()?;
    conn.execute(
        &format!("DELETE FROM {} WHERE uuid=?", table_name(suuid)),
        [uuid],
    )?;
    Ok(())
}

pub fn insert(suuid: &str, uuid: &str, name: &str, data: &str) -> Result<()> {
    let conn = connection()?;

    conn.execute(
        &format!(
            "INSERT INTO {} (uuid, name, data) VALUES (?, ?, ?)",
            table_name(suuid)
        ),
        [uuid, name, data],
    )?;
    Ok(())
}

pub fn update(suuid: &str, uuid: &str, name: &str) -> Result<()> {
    let conn = connection()?;

    conn.execute(
        &format!("UPDATE {} SET name=? WHERE uuid=?", table_name(suuid)),
        [name, uuid],
    )?;

    Ok(())
}

pub fn select(suuid: &str, uuid: &str) -> Result<Option<(String, String)>> {
    let conn = connection()?;

    let mut stmt = conn.prepare(&format!(
        "SELECT name, data FROM {} WHERE uuid='{}'",
        table_name(suuid),
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

pub fn select_all(suuid: &str) -> Result<Vec<(String, String, String)>> {
    let conn = connection()?;

    let mut stmt = conn.prepare(&format!(
        "SELECT uuid, name, data FROM {}",
        table_name(suuid)
    ))?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    Ok(rows.flatten().collect())
}

#[allow(dead_code)]
pub fn is_exist(suuid: &str, uuid: &str) -> Result<bool> {
    let conn = connection()?;
    let cnt = conn.query_row::<i64, _, _>(
        &format!("SELECT 1 FROM {} WHERE uuid='{}'", table_name(suuid), uuid),
        [],
        |r| r.get(0),
    )?;
    Ok(cnt == 1)
}

pub fn is_table_exist(suuid: &str) -> Result<bool> {
    let conn = connection()?;

    let sql = format!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='{}'",
        table_name(suuid)
    );
    let mut stmt = conn.prepare(&sql)?;
    let result = stmt.query_map(params![], |row| row.get::<_, String>(0))?;
    Ok(!result.count().eq(&0))
}

pub fn drop_table(suuid: &str) -> Result<()> {
    let conn = connection()?;
    conn.execute(&format!("DROP TABLE {}", table_name(suuid)), [])?;
    Ok(())
}
