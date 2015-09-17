#[derive(Debug, Copy, Clone)]
pub enum ItemType {
    Commander,
    Bulletin,
    Skin,
    VictoryStrike,
    Decal,
    FacePlate
}

pub struct Item {
    id: u64,
    item_type: ItemType
}

impl Item {
    pub fn new(item_type: ItemType) -> Item {
        Item {
            id: 0,
            item_type: item_type,
        }
    }

    pub fn with_split_id(primary: u32, secondary: u32, item_type: ItemType) -> Item {
        let mut item = Item {
            id: 0,
            item_type: item_type
        };

        item.update_id(primary, secondary);
        item
    }

    pub fn with_whole_id(id: u64, item_type: ItemType) -> Item {
        Item {
            id: id,
            item_type: item_type,
        }
    }

    pub fn update_id(&mut self, primary: u32, secondary: u32) {
        let primary_64 = primary as u64;
        let secondary_64 = secondary as u64;

        self.id = (primary_64 << 32) + secondary_64;
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn item_type(&self) -> ItemType {
        self.item_type
    }
}