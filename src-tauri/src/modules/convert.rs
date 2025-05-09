use crate::building_gadges::to_bg_schematic::ToBgSchematic;
use crate::create::to_create_schematic::ToCreateSchematic;
use crate::data_files::files::FileManager;
use crate::database::db_apis::schematics_api::find_schematic;
use crate::database::db_control::DatabaseState;
use crate::litematica::to_lm_schematic::ToLmSchematic;
use crate::modules::modules_data::convert_data::ConvertData;
use crate::utils::minecraft_data::je_blocks_data::{BlocksData, SubData};
use crate::word_edit::to_we_schematic::ToWeSchematic;
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn get_schematic_convert_data(
    db: State<'_, DatabaseState>,
    file_manager: State<'_, FileManager>,
    id: i64,
) -> anyhow::Result<ConvertData, String> {
    async move {
        let mut conn = db.0.get()?;
        let schematic = find_schematic(&mut conn, id)?;
        let version = schematic.version;
        let sub_version = schematic.sub_type;
        let v_type = schematic.schematic_type;
        let data = file_manager.get_convert_data(id, version, sub_version, v_type)?;
        Ok(data)
    }
    .await
    .map_err(|e: anyhow::Error| e.to_string())
}

#[tauri::command]
pub async fn get_je_blocks(
    je_blocks: State<'_, BlocksData>,
) -> anyhow::Result<Vec<SubData>, String> {
    async move {
        let data = je_blocks.blocks.clone();
        Ok(data)
    }
    .await
    .map_err(|e: anyhow::Error| e.to_string())
}

#[tauri::command]
pub async fn convert(
    db: State<'_, DatabaseState>,
    file_manager: State<'_, FileManager>,
    id: i64,
    schematic_type: i64,
    lm_version: i64,
    we_version: i64,
    bg_version: i64,
    vi_air: bool,
) -> anyhow::Result<bool, String> {
    async move {
        let mut conn = db.0.get()?;
        let schematic = find_schematic(&mut conn, id)?;
        let version = schematic.version;
        let sub_version = schematic.sub_type;
        let v_type = schematic.schematic_type;
        let data = file_manager.get_schematic_data(id, version, sub_version, v_type)?;
        match schematic_type {
            1 => {
                let data = ToCreateSchematic::new(&data).create_schematic(vi_air);
                file_manager.save_nbt_value(
                    id,
                    data,
                    version,
                    sub_version,
                    schematic_type as i32,
                    true,
                )?;
            }
            2 => {
                let data = ToLmSchematic::new(&data).lm_schematic(lm_version as i32);
                file_manager.save_nbt_value(
                    id,
                    data,
                    version,
                    sub_version,
                    schematic_type as i32,
                    true,
                )?;
            }
            3 => {
                let data = ToWeSchematic::new(&data).we_schematic(we_version as i32)?;
                file_manager.save_nbt_value(
                    id,
                    data,
                    version,
                    we_version as i32,
                    schematic_type as i32,
                    true,
                )?;
            }
            4 => {
                let data = ToBgSchematic::new(&data).bg_schematic()?;
                file_manager.save_json_value(
                    id,
                    data,
                    version,
                    bg_version as i32,
                    schematic_type as i32,
                )?;
            }
            //5 => {}
            _ => {
                anyhow::bail!("unknown schematic type: {}", schematic_type);
            }
        }
        Ok(true)
    }
    .await
    .map_err(|e: anyhow::Error| e.to_string())
}
