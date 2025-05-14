use crate::building_gadges::bg_schematic_data::JsonData;
use crate::database::db_control::DatabaseState;
use crate::database::db_data::{PaginatedResponse, Schematic};
use crate::utils::schematic_data::SchematicError;
use anyhow::Result;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use tauri::State;

pub fn delete_schematic_data(
    conn: &mut PooledConnection<SqliteConnectionManager>,
    id: i64,
) -> Result<i64> {
    let tx = conn.transaction()?;
    tx.execute(
        r#"UPDATE schematics
        SET is_deleted = TRUE,
            updated_at = strftime('%Y-%m-%d %H:%M:%f', 'now')
        WHERE id = ?
        AND is_deleted = FALSE"#,
        params![id],
    )?;
    tx.commit()?;
    Ok(id)
}

pub fn update_schematic(
    conn: &mut PooledConnection<SqliteConnectionManager>,
    schematic: Schematic,
) -> Result<i64> {
    let tx = conn.transaction()?;

    tx.execute(
        r#"UPDATE schematics
        SET
            name = ?1,
            description = ?2,
            type = ?3,
            sub_type = ?4,
            sizes = ?5,
            user = ?6,
            version = ?7,
            game_version = ?8,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?9"#,
        params![
            schematic.name,
            schematic.description,
            schematic.schematic_type,
            schematic.sub_type,
            schematic.sizes,
            schematic.user,
            schematic.version,
            schematic.game_version,
            schematic.id
        ],
    )?;

    tx.commit()?;

    Ok(schematic.id)
}

pub fn update_schematic_name(
    conn: &mut PooledConnection<SqliteConnectionManager>,
    name: String,
    description: String,
    schematic_id: i64
) -> Result<i64> {
    let tx = conn.transaction()?;

    tx.execute(
        r#"UPDATE schematics
        SET
            name = ?1,
            description = ?2,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?3"#,
        params![
            name,
            description,
            schematic_id
        ],
    )?;

    tx.commit()?;

    Ok(schematic_id)
}
pub fn new_schematic(
    mut conn: &mut PooledConnection<SqliteConnectionManager>,
    schematic: Schematic,
) -> Result<i64> {
    let tx = conn.transaction()?;
    tx.execute(
        r#"INSERT INTO schematics (
            name, description, type, sub_type,
            sizes, user, version_list, game_version
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"#,
        params![
            schematic.name,
            schematic.description,
            schematic.schematic_type,
            schematic.sub_type,
            schematic.sizes,
            schematic.user,
            schematic.version_list,
            schematic.game_version
        ],
    )?;
    let rowid = tx.last_insert_rowid();
    tx.commit()?;

    Ok(rowid)
}

pub fn find_schematic(
    mut conn: &mut PooledConnection<SqliteConnectionManager>,
    id: i64,
) -> Result<Schematic> {
    let tx = conn.transaction()?;
    let schematic = tx.query_row(
        "SELECT * FROM schematics WHERE id = ? AND is_deleted = FALSE",
        [id],
        |row| {
            Ok(Schematic {
                id: row.get("id")?,
                name: row.get("name")?,
                description: row.get("description")?,
                schematic_type: row.get("type")?,
                sub_type: row.get("sub_type")?,
                is_deleted: row.get("is_deleted")?,
                sizes: row.get("sizes")?,
                user: row.get("user")?,
                is_upload: row.get("is_upload")?,
                version: row.get("version")?,
                version_list: row.get("version_list")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                game_version: row.get("game_version")?,
            })
        },
    );
    tx.commit()?;
    Ok(schematic?)
}

pub fn get_schematic_version(
    mut conn: &mut PooledConnection<SqliteConnectionManager>,
    id: i64,
) -> Result<i32> {
    let tx = conn.transaction()?;
    let schematic = tx.query_row(
        "SELECT * FROM schematics WHERE id = ? AND is_deleted = FALSE",
        [id],
        |row| Ok(row.get("version")?),
    );
    tx.commit()?;
    Ok(schematic?)
}

#[tauri::command]
pub fn add_schematic(db: State<'_, DatabaseState>, schematic: Schematic) -> Result<i64, String> {
    let mut conn = db.0.get().map_err(|e| e.to_string())?;

    let new = new_schematic(&mut conn, schematic).map_err(|e| e.to_string())?;
    Ok(new)
}

#[tauri::command]
pub fn update_schematic_tauri(
    db: State<'_, DatabaseState>,
    schematic: Schematic,
) -> Result<i64, String> {
    let mut conn = db.0.get().map_err(|e| e.to_string())?;

    let new = update_schematic(&mut conn, schematic).map_err(|e| e.to_string())?;
    Ok(new)
}

#[tauri::command]
pub fn get_schematic(db: State<'_, DatabaseState>, id: i64) -> Result<Schematic, String> {
    let mut conn = db.0.get().map_err(|e| e.to_string())?;
    let schematic = find_schematic(&mut conn, id);
    Ok(schematic.map_err(|e| e.to_string())?)
}
#[tauri::command]
pub fn get_schematics(
    db: State<'_, DatabaseState>,
    filter: &str,
    page: i32,
    page_size: i32,
) -> Result<PaginatedResponse<Schematic>, String> {
    let conn = db.0.get().map_err(|e| e.to_string())?;
    let page = page.max(1);
    let page_size = page_size.clamp(1, 100);

    let offset = (page - 1) * page_size;
    let search_pattern = if filter.is_empty() {
        "".to_string()
    } else {
        format!("%{}%", filter)
    };
    let mut stmt = conn
        .prepare(
            r#"
        SELECT * FROM schematics
        WHERE
            (?1 = '' OR
            (name LIKE ?1 OR description LIKE ?1))
            AND is_deleted = FALSE
        ORDER BY created_at DESC
        LIMIT ?2 OFFSET ?3
        "#,
        )
        .map_err(|e| e.to_string())?;

    let schematics = stmt
        .query_map(
            rusqlite::params![search_pattern, page_size, offset],
            |row| {
                Ok(Schematic {
                    id: row.get("id")?,
                    name: row.get("name")?,
                    description: row.get("description")?,
                    schematic_type: row.get("type")?,
                    sub_type: row.get("sub_type")?,
                    is_deleted: row.get("is_deleted")?,
                    sizes: row.get("sizes")?,
                    user: row.get("user")?,
                    is_upload: row.get("is_upload")?,
                    version: row.get("version")?,
                    version_list: row.get("version_list")?,
                    created_at: row.get("created_at")?,
                    updated_at: row.get("updated_at")?,
                    game_version: row.get("game_version")?,
                })
            },
        )
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(PaginatedResponse {
        data: schematics,
        page,
        page_size,
    })
}

#[tauri::command]
pub fn delete_schematic_database(db: State<'_, DatabaseState>, id: i64) -> Result<i64, String> {
    let mut conn = db.0.get().map_err(|e| e.to_string())?;

    let new = delete_schematic_data(&mut conn, id).map_err(|e| e.to_string())?;
    Ok(new)
}
