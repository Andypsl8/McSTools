use std::collections::{BTreeMap};
use std::fs::File;
use std::io::{BufReader};
use std::sync::Arc;
use flate2::read::GzDecoder;
use fastnbt::{self, Value, Value::Compound};
use crate::utils::block_state_pos_list::{BlockData, BlockId, BlockPos, BlockStatePosList};
use crate::utils::schematic_data::{SchematicData, SchematicError};
use crate::utils::tile_entities::TileEntitiesList;
#[derive(Debug)]
pub struct CreateSchematic {
    nbt: Value,
}

impl CreateSchematic {
    pub fn new(file_path: &str) -> Result<Self, SchematicError> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let decoder = GzDecoder::new(reader);

        let nbt: Value = fastnbt::from_reader(decoder)?;

        if let Compound(_) = &nbt {
            Ok(Self { nbt })
        } else {
            Err(SchematicError::InvalidFormat("Root is not a Compound"))
        }
    }

    pub fn get_pos_list(&self) -> Result<&Vec<Value>, SchematicError> {
        if let Compound(root) = &self.nbt {
            root.get("blocks")
                .and_then(|v| match v {
                    Value::List(list) => Some(list),
                    _ => None
                })
                .ok_or(SchematicError::InvalidFormat("NotFound Blocks is not a list"))
        } else {
            Err(SchematicError::InvalidFormat("Root is not a Compound"))
        }
    }

    pub fn get_size(&self) -> Result<&Vec<Value>, SchematicError> {
        if let Compound(root) = &self.nbt {
            root.get("size")
                .and_then(|v| match v {
                    Value::List(list) => Some(list),
                    _ => None
                })
                .ok_or(SchematicError::InvalidFormat("NotFound Size is not a IntArray"))
        } else {
            Err(SchematicError::InvalidFormat("Root is not a Compound"))
        }
    }

    pub fn get_palette(&self) -> Result<&Vec<Value>, SchematicError> {
        if let Compound(root) = &self.nbt {
            root.get("palette")
                .and_then(|v| match v {
                    Value::List(list) => Some(list),
                    _ => None
                })
                .ok_or(SchematicError::InvalidFormat("NotFound Size is not a IntArray"))
        } else {
            Err(SchematicError::InvalidFormat("Root is not a Compound"))
        }
    }

    pub fn get_entities(&self) -> Result<&Vec<Value>, SchematicError> {
        if let Compound(root) = &self.nbt {
            root.get("entities")
                .and_then(|v| match v {
                    Value::List(list) => Some(list),
                    _ => None
                })
                .ok_or(SchematicError::InvalidFormat("NotFound Size is not a IntArray"))
        } else {
            Err(SchematicError::InvalidFormat("Root is not a Compound"))
        }
    }

    fn parse_palette(&self) -> Result<Vec<Arc<BlockData>>, SchematicError> {
        let palette_list = self.get_palette()?;

        let mut palette = Vec::with_capacity(palette_list.len());

        for entry in palette_list {
            let Compound(root) = entry else { todo!() };
            let name = root.get("Name")
                .and_then(|v| v.as_str())
                .map(|s| Arc::<str>::from(s))
                .unwrap_or_else(|| Arc::from("minecraft:air"));


            let mut properties = BTreeMap::new();
            if let Some(Compound(prop_map)) = root.get("Properties") {
                for (k, v) in prop_map {
                    if let Value::String(s) = v {
                        properties.insert(
                            Arc::<str>::from(k.as_str()),
                            Arc::<str>::from(s.as_str())
                        );
                    }
                }
            }

            palette.push(Arc::new(BlockData {
                id: BlockId { name },
                properties,
            }));
        }

        Ok(palette)
    }

    pub fn get_blocks_pos(&self) -> Result<SchematicData, SchematicError> {
        let mut block_list = BlockStatePosList::default();
        let tile_entities = TileEntitiesList::default();
        let blocks = self.get_pos_list()?;
        let palette = self.parse_palette()?;
        for block in blocks.iter() {

            let pos = if let Compound(compound) = block {
                let pos_values = compound.get("pos")
                    .ok_or(SchematicError::InvalidFormat("Missing pos field"))?;

                let coords = match pos_values {
                    Value::List(list) => list.iter()
                        .filter_map(|v| {
                            match v {
                                Value::Int(n) => Some(*n as i32),
                                _ => None
                            }
                        })
                        .collect::<Vec<i32>>(),
                    Value::IntArray(arr) => arr.to_vec(),
                    _ => return Err(SchematicError::InvalidFormat("Invalid pos type"))
                };

                if coords.len() != 3 {
                    return Err(SchematicError::InvalidFormat("Position requires 3 coordinates"));
                }

                BlockPos {
                    x: coords[0],
                    y: coords[1],
                    z: coords[2],
                }
            } else {
                return Err(SchematicError::InvalidFormat("Block entry is not a compound"));
            };

            let state_id_value = if let Compound(compound) = block {
                compound.get("state")
                    .ok_or(SchematicError::InvalidFormat("Missing state field"))?
            } else {
                return Err(SchematicError::InvalidFormat("Block entry is not a compound"));
            };
            let state_id = match state_id_value {
                Value::Int(n) => *n as usize,
                _ => return Err(SchematicError::InvalidFormat("State ID must be integer")),
            };
            let block_data = &palette[state_id];
            block_list.add(pos, Arc::clone(block_data))
        }

        Ok(SchematicData::new(block_list, tile_entities))
    }
}

pub fn extract_namespace(input: &str) -> Result<(&str, &str), SchematicError> {
    input.split_once(':')
        .ok_or(SchematicError::InvalidFormat("Invalid namespace format"))
}