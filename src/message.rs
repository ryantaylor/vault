use data::ticks::Tick;
use data::ticks::Tick::Message as MessageEnum;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "magnus", magnus::wrap(class = "Vault::Message"))]
pub struct Message {
    tick: u32,
    message: String,
}

impl Message {
    pub fn tick(&self) -> u32 {
        self.tick
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}

pub fn messages_from_data(data: Vec<&Tick>, player_name: &str) -> Vec<Message> {
    let mut tick_count = 0;

    data.iter()
        .flat_map(|tick| {
            tick_count += 1;

            match tick {
                MessageEnum(message_tick) => message_tick
                    .messages
                    .iter()
                    .map(|message| {
                        if message.name == player_name {
                            Some(Message {
                                tick: tick_count,
                                message: message.message.clone(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect(),
                _ => vec![None],
            }
        })
        .filter(|entry| matches!(entry, Some(_)))
        .map(|entry| match entry {
            Some(message) => message,
            None => panic!(),
        })
        .collect()
}
