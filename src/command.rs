//! A module containing a representation of all possible commands and actions in CoH2 replays.
//!
//! This module is used in conjunction with replay to parse actions within ticks based on the
//! command they encode.

use std::mem;

/// This type represents a single Company of Heroes 2 player command.

#[derive(Debug, RustcEncodable)]
pub struct Command {
    pub player_id: u8,
    pub tick: u32,
    pub command_type: CmdType,
    pub blueprint: Blueprint,
    pub entity_id: u32,
    pub x: u32,
    pub y: u32,
    pub bytes: Vec<u8>,
}

impl Command {

    /// Constructs a new, empty Command.

    pub fn new(tick: u32, command_type: CmdType) -> Command {
        Command {
            player_id: 0,
            tick: tick,
            command_type: command_type,
            blueprint: Blueprint::Ebps,
            entity_id: 0,
            x: 0,
            y: 0,
            bytes: Vec::new(),
        }
    }
}

/// This type contains a numerical u8 representation of every command/action possible in a CoH2 command
/// sequence. Contents of this enum provided by Relic Entertainment.

#[derive(Debug, Copy, Clone, RustcEncodable)]
#[repr(u8)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum CmdType {
    //[EntityCommandType]
    CMD_DefaultAction = 0,
    CMD_Stop = 1,
    CMD_Destroy = 2,
    CMD_BuildSquad = 3,
    CMD_InstantBuildSquad = 4,
    CMD_CancelProduction = 5,
    CMD_BuildStructure = 6,
    CMD_Move = 7,
    CMD_Face = 8,
    CMD_Attack = 9,
    CMD_AttackMove = 10,
    CMD_RallyPoint = 11,
    CMD_Capture = 12,
    CMD_Ability = 13,
    CMD_Evacuate = 14,
    CMD_Upgrade = 15,
    CMD_InstantUpgrade = 16,
    CMD_ChooseResource = 17,
    CMD_Load = 18,
    CMD_Unload = 19,
    CMD_UnloadSquads = 20,
    CMD_AttackStop = 21,
    CMD_AttackForced = 22,
    CMD_SetHoldHeading = 23,
    CMD_Halt = 24,
    CMD_Fidget = 25,
    CMD_Paradrop = 26,
    CMD_DefuseMine = 27,
    CMD_Casualty = 28,
    CMD_Death = 29,
    CMD_InstantDeath = 30,
    CMD_Projectile = 31,
    CMD_PlaceCharge = 32,
    CMD_BuildEntity = 33,
    CMD_RescueCasualty = 34,
    CMD_AttackFromHold = 35,
    CMD_Vault = 36,
    CMD_COUNT = 37,

    //[SquadCommandType]
    //SCMD_DefaultAction = 0
    SCMD_Move = 38,
    SCMD_Stop = 39,
    SCMD_Destroy = 40,
    SCMD_BuildStructure = 41,
    SCMD_Capture = 42,
    SCMD_Attack = 43,
    SCMD_ReinforceUnit = 44,
    SCMD_Upgrade = 45,
    SCMD_CancelProduction = 46,
    SCMD_AttackMove = 47,
    SCMD_Ability = 48,
    SCMD_Load = 49,
    SCMD_InstantLoad = 50,
    SCMD_UnloadSquads = 51,
    SCMD_Unload = 52,
    SCMD_SlotItemRemove = 53,
    SCMD_Retreat = 54,
    SCMD_CaptureTeamWeapon = 55,
    SCMD_SetMoveType = 56,
    SCMD_InstantReinforceUnit = 57,
    SCMD_InstantUpgrade = 58,
    SCMD_SetCamouflageStance = 59,
    SCMD_PlaceCharge = 60,
    SCMD_DefuseCharge = 61,
    SCMD_PickUpSlotItem = 62,
    SCMD_DefuseMine = 63,
    SCMD_DoPlan = 64,
    SCMD_Patrol = 65,
    SCMD_Surprise = 66,
    SCMD_InstantSetupTeamWeapon = 67,
    SCMD_AbandonTeamWeapon = 68,
    SCMD_StationaryAttack = 69,
    SCMD_RevertFieldSupport = 70,
    SCMD_Face = 71,
    SCMD_BuildSquad = 72,
    SCMD_RallyPoint = 73,
    SCMD_RescueCasualty = 74,
    SCMD_Recrew = 75,
    SCMD_Merge = 76,
    SCMD_Pilfer = 77,
    SCMD_COUNT = 78,

    //[PlayerCommandType]
    PCMD_ConstructStructure = 79,
    PCMD_ManpowerDonation = 80,
    PCMD_FuelDonation = 81,
    PCMD_MunitionDonation = 82,
    PCMD_CheatResources = 83,
    PCMD_CheatRevealAll = 84,
    PCMD_CheatKillSelf = 85,
    PCMD_Ability = 86,
    PCMD_CheatBuildTime = 87,
    PCMD_CriticalHit = 88,
    PCMD_Upgrade = 89,
    PCMD_InstantUpgrade = 90,
    PCMD_ConstructFence = 91,
    PCMD_ConstructField = 92,
    PCMD_UpgradeRemove = 93,
    PCMD_SlotItemRemove = 94,
    PCMD_CancelProduction = 95,
    PCMD_DetonateCharges = 96,
    PCMD_AIPlayer = 97,
    PCMD_AIPlayer_ObjectiveNotification = 98,
    PCMD_SetCommander = 99,
    PCMD_Surrender = 100,
    PCMD_WaitObjectDone = 101,
    PCMD_BroadcastMessage = 102,
    PCMD_COUNT = 103,

    //[DataCommandtype]
    DCMD_DataCommand1 = 104, // 13 data bytes
    DCMD_DataCommand2 = 105, // 10 data bytes
    DCMD_COUNT = 106,
}

impl CmdType {

    /// Converts a numerical representation of a CmdType enum into the correct enum value by
    /// unsafely transmuting the numerical representation into the CmdType type.

    pub fn from_u8(n: u8) -> Option<CmdType> {
        if n <= 106 {
            Some(unsafe { mem::transmute(n) })
        } else {
            None
        }
    }
}

/// This type contains a numerical u8 representation of entity type IDs as given in Relic's entity
/// blueprints for Company of Heroes 2.

#[derive(Debug, Copy, Clone, RustcEncodable)]
#[repr(u8)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum Blueprint {
    Ebps = 0,
    Sbps = 1,
    Formation = 2,
    Squad_Formation = 3,
    Turn_Plan = 4,
    Racebps = 5,
    Upgrade = 6,
    Ability = 7,
    Tuning = 8,
    Material = 9,
    Weapon = 10,
    Slot_Item = 11,
    Pass_Type = 12,
    Move_Type = 13,
    Hit_Material = 14,
    Critical = 15,
    Camouflage_Stance_Type = 16,
    Posture = 17,
    Ui_Selection = 18,
    Ui_Territory = 19,
    Ui_Tacticalmap = 20,
    Commander = 21,
    Aura = 22,
    Engine_Type = 23,
    Intel_Bulletin = 24,
    Leveling = 25,
    Thq_Store = 26,
    Construction_Menu = 27,
    Post_Game_Achievement = 28,
    Ui_Reticule = 29,
    Load_Tips = 30,
    Load_Tip_Group = 31,
    Skin_Pack = 32,
    Ribbon = 33,
    Ui_Rectangle = 34,
    Ui_Setfacing = 35,
    Ai_Economy = 36,
    Ai_Economy_Group = 37,
    Ai_Personality = 38,
    Ai_Settings = 39,
    Faceplate = 40,
    Fatality = 41,
    Tow_Operation = 42,
    Campaign = 43,
    Ribbon_Group = 44,
    Achievement = 45,
    Vehicle_Decal = 46,
    Building_Info = 47,
    Weapon_Building_Damage = 48,
    Ai_Ability = 49,
    Commander_Ability = 50,
    Ai_Squad = 51,
    Inventory_Item_Category = 52,
    Army = 53,
}

impl Blueprint {

    /// Converts a numerical representation of a Blueprint enum into the correct enum value by
    /// unsafely transmuting the numerical representation into the Blueprint type.

    pub fn from_u8(n: u8) -> Option<Blueprint> {
        if n <= 53 {
            Some(unsafe { mem::transmute(n) })
        } else {
            None
        }
    }
}