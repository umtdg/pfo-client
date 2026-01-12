use serde::Serialize;

#[derive(Serialize)]
pub struct NoneSerialize;

impl NoneSerialize {
    pub fn new() -> Option<Self> {
        None
    }
}

pub fn none_serialize() -> Option<NoneSerialize> {
    NoneSerialize::new()
}
