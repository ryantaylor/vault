//! A module containing a representation of items in CoH2 that can be equipped, as represented in
//! CoH2 replay files.

/// This type contains the types of all potentially equipped items that can be parsed out of CoH2
/// replays.

#[derive(Debug, Copy, Clone, RustcEncodable)]
pub enum ItemType {
    Commander,
    Bulletin,
    Skin,
    VictoryStrike,
    Decal,
    FacePlate
}

/// This type contains a parsed representation of an item that can be equipped in a CoH2 replay.

#[derive(Debug, RustcEncodable)]
pub struct Item {
    pub id: u64,
    pub item_type: ItemType
}

impl Item {

    /// Constructs a new Item with an empty ID and the given ItemType.

    pub fn new(item_type: ItemType) -> Item {
        Item {
            id: 0,
            item_type: item_type,
        }
    }

    /// Constructs a new Item by combining the give u32 primary and secondary IDs into a single
    /// unique u64 ID, and passing through the given ItemType.

    pub fn with_split_id(primary: u32, secondary: u32, item_type: ItemType) -> Item {
        let mut item = Item {
            id: 0,
            item_type: item_type
        };

        item.update_id(primary, secondary);
        item
    }

    /// Constructs a new Item with the given ID and ItemType. This function simply passes through
    /// the given ID to the Item without any transformation.

    pub fn with_whole_id(id: u64, item_type: ItemType) -> Item {
        Item {
            id: id,
            item_type: item_type,
        }
    }

    /// Combines u32 primary and secondary IDs into a single unique u64 ID by shifting the primary
    /// ID 32 bits to the left and then adding the two IDs together.

    pub fn update_id(&mut self, primary: u32, secondary: u32) {
        let primary_64 = primary as u64;
        let secondary_64 = secondary as u64;

        self.id = (primary_64 << 32) + secondary_64;
    }
}