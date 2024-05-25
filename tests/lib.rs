//! `vault` library integration tests.

extern crate vault;

use uuid::{uuid, Uuid};
use vault::{GameType, Replay};

#[test]
fn parse_success() {
    let data = include_bytes!("../replays/USvDAK_v10612.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok());
    let unwrapped = replay.unwrap();
    assert_eq!(unwrapped.version(), 10612);
    assert_eq!(
        unwrapped
            .players()
            .iter()
            .map(|player| { player.name() })
            .collect::<Vec<&str>>(),
        vec!["madhax", "Quixalotl"]
    );
    assert_eq!(unwrapped.mod_uuid(), Uuid::nil());
    assert_eq!(unwrapped.game_type(), GameType::Multiplayer);
    assert_eq!(unwrapped.matchhistory_id(), Some(5569487));
}

#[test]
fn parse_failure() {
    let data = [1, 2, 3];
    let replay = Replay::from_bytes(&data);
    assert!(replay.is_err())
}

#[test]
fn parse_success_ai() {
    let data = include_bytes!("../replays/vs_ai.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok());
    let unwrapped = replay.unwrap();
    assert_eq!(unwrapped.version(), 21283);
    assert_eq!(
        unwrapped
            .players()
            .iter()
            .map(|player| { player.name() })
            .collect::<Vec<&str>>(),
        vec!["Janne252", "CPU - Standard"]
    );
    assert_eq!(
        unwrapped.mod_uuid(),
        uuid!("385d9810-96ba-4ece-9040-8281db65174e")
    );
    assert_eq!(unwrapped.game_type(), GameType::Skirmish);
    assert_eq!(unwrapped.matchhistory_id(), None);
}

#[test]
fn parse_weird_description() {
    let data = include_bytes!("../replays/weird_description.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok());
    let unwrapped = replay.unwrap();
    assert_eq!(unwrapped.map().localized_name_id(), "Twin Beaches ML");
    assert_eq!(unwrapped.map().localized_description_id(), "TB ML");
    assert_eq!(unwrapped.game_type(), GameType::Multiplayer);
    assert_eq!(unwrapped.matchhistory_id(), Some(11782009));
}

#[test]
fn parse_battlegroup() {
    let data = include_bytes!("../replays/USvDAK_v10612.rec");
    let replay = Replay::from_bytes(data).unwrap();
    assert_eq!(
        replay
            .players()
            .iter()
            .map(|player| { player.battlegroup() })
            .collect::<Vec<Option<u32>>>(),
        vec![Some(2072430), Some(196934)]
    );
}

#[test]
fn parse_automatch() {
    let data = include_bytes!("../replays/automatch.rec");
    let replay = Replay::from_bytes(data).unwrap();
    assert_eq!(replay.game_type(), GameType::Automatch);
    assert_eq!(replay.matchhistory_id(), Some(18837622));
}

#[test]
fn parse_custom() {
    let data = include_bytes!("../replays/custom.rec");
    let replay = Replay::from_bytes(data).unwrap();
    assert_eq!(replay.game_type(), GameType::Custom);
    assert_eq!(replay.matchhistory_id(), Some(18838931));
}

#[test]
fn parse_skirmish() {
    let data = include_bytes!("../replays/skirmish.rec");
    let replay = Replay::from_bytes(data).unwrap();
    assert_eq!(replay.game_type(), GameType::Skirmish);
    assert_eq!(replay.matchhistory_id(), None);
}
