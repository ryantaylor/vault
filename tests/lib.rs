//! `vault` library integration tests.

extern crate vault;

use std::{
    collections::HashSet,
    fs::{self, File},
    io::Read,
    thread,
};
use uuid::{uuid, Uuid};
use vault::{Command, CommandType, Faction, GameType, Replay, Team};

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

#[test]
fn parse_new_map_chunk() {
    let data = include_bytes!("../replays/one_seven_zero.rec");
    let replay = Replay::from_bytes(data).unwrap();
    assert_eq!(
        replay.map_filename(),
        "data:scenarios\\multiplayer\\desert_airfield_6p_mkii\\desert_airfield_6p_mkii"
    );
    assert_eq!(replay.map_localized_name_id(), "$11233954");
    assert_eq!(replay.map_localized_description_id(), "$11233955");
}

#[test]
fn parse_ai_takeover() {
    let data = include_bytes!("../replays/ai_takeover.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok());
}

#[test]
fn parse_zero_item_player() {
    let data = include_bytes!("../replays/zero_items.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok());
}

#[test]
fn parse_unusual_items_player() {
    let data = include_bytes!("../replays/unusual_items.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok());
}

#[test]
fn parse_unusual_options() {
    let data = include_bytes!("../replays/unusual_options.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok());
}

#[test]
fn parse_one_delimited_options() {
    let data = include_bytes!("../replays/one_delimited_options.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok());
}

#[test]
fn parse_unusual_cpu_items() {
    let data = include_bytes!("../replays/unusual_cpu_items.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok());
}

#[test]
fn parse_unusual_brit_faction() {
    let data = include_bytes!("../replays/unusual_brit_faction.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok());
    let unwrapped = replay.unwrap();
    assert_eq!(
        unwrapped
            .players()
            .iter()
            .map(|player| { player.faction() })
            .collect::<Vec<Faction>>(),
        vec![
            Faction::British,
            Faction::Americans,
            Faction::Wehrmacht,
            Faction::Wehrmacht,
            Faction::AfrikaKorps,
            Faction::Americans
        ]
    );
}

#[test]
fn parse_one_char_options() {
    let data = include_bytes!("../replays/one_char_options.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok());
}

#[test]
fn parse_unusual_team_id() {
    let data = include_bytes!("../replays/unusual_team_id.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok());
    let unwrapped = replay.unwrap();
    assert_eq!(
        unwrapped
            .players()
            .iter()
            .map(|player| { player.team() })
            .collect::<Vec<Team>>(),
        vec![
            Team::First,
            Team::Second,
            Team::First,
            Team::Second,
            Team::First,
            Team::Second,
            Team::First,
            Team::Second
        ]
    );
}

#[test]
#[cfg_attr(not(feature = "regression"), ignore)]
fn regression() {
    let paths = fs::read_dir("replays/regression").unwrap();
    let pathbufs: Vec<_> = paths
        .into_iter()
        .map(|path| path.unwrap().path())
        .filter(|path| path.is_file())
        .collect();
    let chunks = pathbufs.chunks(100);
    let mut results = Vec::new();

    for chunk in chunks {
        let handles: Vec<_> = chunk
            .iter()
            .map(|path| {
                let cloned_path = path.clone();
                thread::spawn(move || {
                    let mut file = File::open(cloned_path.clone()).unwrap();
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer).unwrap();

                    match Replay::from_bytes(&buffer) {
                        Ok(replay) => Ok(replay),
                        Err(_) => Err(format!("failed to parse {:?}", cloned_path)),
                    }
                })
            })
            .collect();

        let mut parse_results: Vec<_> = handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect();
        results.append(&mut parse_results);
    }

    let errs: Vec<_> = results.iter().filter(|result| result.is_err()).collect();
    assert_eq!(errs.len(), 0);
}

#[test]
#[cfg_attr(not(feature = "missing"), ignore)]
fn missing_commands() {
    let paths = fs::read_dir("replays/regression").unwrap();
    let pathbufs: Vec<_> = paths
        .into_iter()
        .map(|path| path.unwrap().path())
        .filter(|path| path.is_file())
        .collect();
    let chunks = pathbufs.chunks(100);
    let mut results: Vec<Result<HashSet<CommandType>, String>> = Vec::new();

    for chunk in chunks {
        let handles: Vec<_> = chunk
            .iter()
            .map(|path| {
                let cloned_path = path.clone();
                thread::spawn(move || {
                    let mut file = File::open(cloned_path.clone()).unwrap();
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer).unwrap();

                    match Replay::from_bytes(&buffer) {
                        Ok(replay) => Ok(HashSet::from_iter(
                            replay
                                .players()
                                .iter()
                                .flat_map(|player| player.commands())
                                .filter_map(|command| {
                                    if let Command::Unknown(data) = command {
                                        if data.index() != 0 {
                                            Some(data.action_type())
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }),
                        )),
                        Err(_) => Err(format!("failed to parse {:?}", cloned_path)),
                    }
                })
            })
            .collect();

        let mut parse_results: Vec<_> = handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect();
        results.append(&mut parse_results);
    }

    let types = results
        .iter()
        .map(|result| result.clone().unwrap_or_default())
        .reduce(|mut acc, set| {
            acc.extend(set);
            acc
        });
    println!("{:?}", types.clone().unwrap());
    assert_eq!(types.unwrap().len(), 0);
}
