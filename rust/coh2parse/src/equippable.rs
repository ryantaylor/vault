#[derive(Debug)]
pub enum ItemType {
    Commander,
    Bulletin,
    Skin,
    VictoryStrike,
    Decal,
    FacePlate
}

pub trait Equippable {
    fn update_id(&mut self, primary: u32, secondary: u32);
    fn id(&self) -> u64;
    fn item_type(&self) -> ItemType;
}

// Commander

pub struct Commander {
    id: u64
}

impl Commander {
    pub fn new() -> Commander {
        Commander {
            id: 0
        }
    }
}

impl Equippable for Commander {
    fn update_id(&mut self, primary: u32, secondary: u32) {
        let primary_64 = primary as u64;
        let secondary_64 = secondary as u64;

        self.id = (primary_64 << 32) + secondary_64;
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn item_type(&self) -> ItemType {
        ItemType::Commander
    }
}

// Bulletin

pub struct Bulletin {
    id: u64
}

impl Bulletin {
    pub fn new() -> Bulletin {
        Bulletin {
            id: 0
        }
    }
}

impl Equippable for Bulletin {
    fn update_id(&mut self, primary: u32, secondary: u32) {
        let primary_64 = primary as u64;
        let secondary_64 = secondary as u64;

        self.id = (primary_64 << 32) + secondary_64;
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn item_type(&self) -> ItemType {
        ItemType::Bulletin
    }
}

// Skin

pub struct Skin {
    id: u64
}

impl Skin {
    pub fn new() -> Skin {
        Skin {
            id: 0
        }
    }
}

impl Equippable for Skin {
    fn update_id(&mut self, primary: u32, secondary: u32) {
        let primary_64 = primary as u64;
        let secondary_64 = secondary as u64;

        self.id = (primary_64 << 32) + secondary_64;
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn item_type(&self) -> ItemType {
        ItemType::Skin
    }
}

// VictoryStrike

pub struct VictoryStrike {
    id: u64
}

impl VictoryStrike {
    pub fn new() -> VictoryStrike {
        VictoryStrike {
            id: 0
        }
    }
}

impl Equippable for VictoryStrike {
    fn update_id(&mut self, primary: u32, secondary: u32) {
        let primary_64 = primary as u64;
        let secondary_64 = secondary as u64;

        self.id = (primary_64 << 32) + secondary_64;
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn item_type(&self) -> ItemType {
        ItemType::VictoryStrike
    }
}

// Decal

pub struct Decal {
    id: u64
}

impl Decal {
    pub fn new() -> Decal {
        Decal {
            id: 0
        }
    }
}

impl Equippable for Decal {
    fn update_id(&mut self, primary: u32, secondary: u32) {
        let primary_64 = primary as u64;
        let secondary_64 = secondary as u64;

        self.id = (primary_64 << 32) + secondary_64;
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn item_type(&self) -> ItemType {
        ItemType::Decal
    }
}

// Decal

pub struct FacePlate {
    id: u64
}

impl FacePlate {
    pub fn new() -> FacePlate {
        FacePlate {
            id: 0
        }
    }
}

impl Equippable for FacePlate {
    fn update_id(&mut self, primary: u32, secondary: u32) {
        let primary_64 = primary as u64;
        let secondary_64 = secondary as u64;

        self.id = (primary_64 << 32) + secondary_64;
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn item_type(&self) -> ItemType {
        ItemType::FacePlate
    }
}