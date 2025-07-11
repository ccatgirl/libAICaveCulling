use crate::entity::block_entity::{
    BlockEntity, BlockEntityType, HasBlockEntity, LockableContainer, LootTable,
};
use crate::entity::parsing::BlockEntityHandler;
use lodestone_common::util::McVersion;
use quartz_nbt::{NbtCompound, NbtList};

pub struct AlphaBlockEntityParser;

impl BlockEntityHandler for AlphaBlockEntityParser {
    fn parse(version: McVersion, nbt: &NbtCompound) -> Option<BlockEntity> {
        // first, parse nbt to get the name/tile id of the block entity
        // then once that's done, you can have a match statement to read the tile entity a certain way based on the version given.
        // but if it's not a massive change, then you can ignore the version and maybe like return a default.

        // so when you match against ID to figure out what block entity you need to parse
        // you parse it's specific data, and give that enum into the BlockEntity you created

        // this can totally be put into a common reader here
        let id: &str = nbt.get("id").expect("Failed to parse tile entity ID!");
        let x: i32 = nbt.get("x").expect("Failed to parse tile entity posX!");
        let y: i32 = nbt.get("y").expect("Failed to parse tile entity posY!");
        let z: i32 = nbt.get("z").expect("Failed to parse tile entity posZ!");

        let data: HasBlockEntity = match id {
            "Chest" => {
                // let items: NbtList = nbt.get("items").expect("Failed to parse tile entity items!");

                let block_entity = BlockEntityType::Chest {
                    items: vec![],
                    lockable_container: LockableContainer { lock: false },
                    loot_table: LootTable {
                        name: "".to_string(),
                        seed: 0,
                    },
                };

                HasBlockEntity::Found(block_entity)
            }
            // if there's one that isn't done by us (NotFound) then you'll want to return a compound with the keys specific to that tile entity inside.
            &_ => HasBlockEntity::NotFound(id.to_string(), nbt.clone()),
        };

        Some(BlockEntity {
            id: id.to_string(),
            x,
            y,
            z,
            components: Default::default(),
            keep_packed: false,
            data,
        })
    }

    fn write(version: McVersion, block_entity: BlockEntity) -> NbtCompound {
        let mut nbt = NbtCompound::new();
        nbt.insert("id", &block_entity.id);
        nbt.insert("x", block_entity.x);
        nbt.insert("y", block_entity.y);
        nbt.insert("z", block_entity.z);

        match version {
            McVersion::Alpha1_2_6 => {
                match block_entity.id.as_str() {
                    "Chest" => {
                        match &block_entity.data {
                            HasBlockEntity::Found(BlockEntityType::Chest { items, .. }) => {
                                let mut item_list = NbtList::new();
                                for item in items.iter() {
                                    let mut item_tag = NbtCompound::new();
                                    item_tag.insert("id", item.id as i16);
                                    item_tag.insert("Slot", item.slot);
                                    item_tag.insert("Count", item.count as i16);
                                    item_list.push(item_tag);
                                }
                                nbt.insert("items", item_list);
                            }
                            &HasBlockEntity::NotFound(_, _) => {
                                // TO BE IMPLEMENTED!
                            }
                            _ => {
                                // TO BE IMPLEMENTED!
                            }
                        }
                    }
                    _ => {
                        // TO BE IMPLEMENTED!
                    }
                }
            }
            _ => todo!(),
        }

        nbt
    }
}
