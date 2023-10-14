use super::*;

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct DataRate {
    pub rate: u8,
}

impl Register for DataRate {
    fn id() -> u8 {
        0x0E
    }
}

impl ReadableRegister<u8> for DataRate {}
impl WritableRegister<u8> for DataRate {}

impl From<u8> for DataRate {
    fn from(rate: u8) -> Self {
        Self { rate }
    }
}

impl Into<u8> for DataRate {
    fn into(self) -> u8 {
        self.rate
    }
}
