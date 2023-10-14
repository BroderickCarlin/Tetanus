use super::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum MovingAverage {
    TwoBit,
    FourBit,
    EightBit,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Tx1 {
    // Moving average for non-filter select
    pub moving_average: Option<MovingAverage>,
    pub data_invert: bool,
    pub modulation_enable: bool,
    pub filter_enable: bool,
    pub fdp: u8,
}

impl Default for Tx1 {
    fn default() -> Self {
        Self {
            moving_average: None,
            data_invert: false,
            modulation_enable: true,
            filter_enable: false,
            fdp: 0b110,
        }
    }
}

impl Register for Tx1 {
    fn id() -> u8 {
        0x14
    }
}

impl WritableRegister<u8> for Tx1 {}

impl Into<u8> for Tx1 {
    fn into(self) -> u8 {
        u8::from(self.data_invert) << 5
            | u8::from(self.modulation_enable) << 4
            | u8::from(self.filter_enable) << 3
            | self.fdp.min(0b111)
            | match self.moving_average {
                None => 0b00,
                Some(MovingAverage::TwoBit) => 0b01,
                Some(MovingAverage::FourBit) => 0b10,
                Some(MovingAverage::EightBit) => 0b11,
            } << 6
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Tx2 {
    // Moving average for non-filter select
    pub fd: u8,
}

impl Default for Tx2 {
    fn default() -> Self {
        Self { fd: 0b01011 }
    }
}

impl Register for Tx2 {
    fn id() -> u8 {
        0x15
    }
}

impl WritableRegister<u8> for Tx2 {}

impl Into<u8> for Tx2 {
    fn into(self) -> u8 {
        self.fd.min(0b11111) | 0b0010_0000
    }
}
