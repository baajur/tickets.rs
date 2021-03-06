use serde::{Serialize, Deserialize};
use super::Opcode;

#[derive(Serialize, Deserialize, Debug)]
pub struct Heartbeat {
    #[serde(rename = "op")]
    opcode: Opcode,

    #[serde(rename = "d")]
    seq: Option<usize>,
}

impl Heartbeat {
    pub fn new(seq: Option<usize>) -> Heartbeat {
        Heartbeat {
            opcode: Opcode::Heartbeat,
            seq,
        }
    }
}
