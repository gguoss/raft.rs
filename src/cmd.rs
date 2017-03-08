use bincode::{serialize, deserialize, SizeLimit};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Command {
    SPAWN_BLK,
}

pub fn encode(cmd: &Command) -> Vec<u8> {
    serialize(cmd, SizeLimit::Infinite).unwrap()
}

pub fn decode(bin: &[u8]) -> Command {
    deserialize(bin).unwrap()
}
