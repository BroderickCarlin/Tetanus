use super::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Fifo1 {
    /// FIFO End Pointer for TX FIFO and Rx FIFO
    pub end_pointer: u8,
}

impl Default for Fifo1 {
    fn default() -> Self {
        Self {
            end_pointer: 0b0011_1111,
        }
    }
}

impl Register for Fifo1 {
    fn id() -> u8 {
        0x03
    }
}

impl WritableRegister<u8> for Fifo1 {}

impl Into<u8> for Fifo1 {
    fn into(self) -> u8 {
        self.end_pointer
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Fifo2 {
    /// FIFO Pointer Margin
    pub margin: u8,
    /// Used for Segment FIFO, Refer to chapter 16 of the datasheet for details
    pub segment: u8,
}

impl Default for Fifo2 {
    fn default() -> Self {
        Self {
            margin: 1,
            segment: 0,
        }
    }
}

impl Register for Fifo2 {
    fn id() -> u8 {
        0x04
    }
}

impl WritableRegister<u8> for Fifo2 {}

impl Into<u8> for Fifo2 {
    fn into(self) -> u8 {
        (self.segment & 0b0011_1111) | (self.margin << 6)
    }
}
