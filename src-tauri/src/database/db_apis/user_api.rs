use crate::database::db_control::DatabaseState;
use crate::database::db_data::{Schematic, UserData};
use anyhow::{Context, Result};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OptionalExtension};
use tauri::State;

pub fn add_user_schematic(conn: &mut PooledConnection<SqliteConnectionManager>) -> Result<i64> {
    let tx = conn.transaction()?;
    tx.execute(
        "UPDATE user_data SET schematics = schematics + 1 WHERE id = 1",
        [],
    )?;

    let new_value: i64 =
        tx.query_row("SELECT schematics FROM user_data WHERE id = 1", [], |row| {
            row.get(0)
        })?;

    tx.commit()?;

    Ok(new_value)
}

pub fn add_cloud(conn: &mut PooledConnection<SqliteConnectionManager>) -> Result<i64> {
    let tx = conn.transaction()?;
    tx.execute("UPDATE user_data SET cloud = cloud + 1 WHERE id = 1", [])?;

    let new_value: i64 = tx.query_row("SELECT cloud FROM user_data WHERE id = 1", [], |row| {
        row.get(0)
    })?;

    tx.commit()?;

    Ok(new_value)
}

#[tauri::command]
pub fn get_user_data(db: State<'_, DatabaseState>) -> Result<UserData, String> {
    let conn = db.0.get().map_err(|e| e.to_string())?;
    Ok(conn
        .query_row("SELECT * FROM user_data WHERE id = 1", [], |row| {
            Ok(UserData {
                id: row.get("id")?,
                nickname: row.get("nickname")?,
                avatar: row.get("avatar")?,
                qq: row.get("qq")?,
                access_token: row.get("accessToken")?,
                openid: row.get("openid")?,
                schematics: row.get("schematics")?,
                cloud: row.get("cloud")?,
            })
        })
        .map_err(|e| e.to_string())?)
}
