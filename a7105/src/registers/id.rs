use super::*;

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct IdData {
    pub id: u32,
}

impl Register for IdData {
    fn id() -> u8 {
        0x06
    }
}

impl ReadableRegister<u32> for IdData {}
impl WritableRegister<u32> for IdData {}

impl From<u32> for IdData {
    fn from(id: u32) -> Self {
        Self { id }
    }
}

impl Into<u32> for IdData {
    fn into(self) -> u32 {
        self.id
    }
}
