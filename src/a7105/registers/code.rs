use super::*;

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum IdLength {
    Two,
    #[default]
    Four,
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum PreambleLength {
    One,
    Two,
    Three,
    #[default]
    Four,
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct Code1 {
    pub data_whitening_enabled: bool,
    pub fec_enabled: bool,
    pub crc_enabled: bool,
    pub id_length: IdLength,
    pub preable_length: PreambleLength,
}

impl Register for Code1 {
    fn id() -> u8 {
        0x1F
    }
}

impl WritableRegister<u8> for Code1 {}

impl Into<u8> for Code1 {
    fn into(self) -> u8 {
        0 | if self.data_whitening_enabled {
            0b0010_0000
        } else {
            0
        } | if self.fec_enabled { 0b0001_0000 } else { 0 }
            | if self.crc_enabled { 0b0000_1000 } else { 0 }
            | if self.id_length == IdLength::Four {
                0b0000_0100
            } else {
                0
            }
            | match self.preable_length {
                PreambleLength::One => 0,
                PreambleLength::Two => 0b01,
                PreambleLength::Three => 0b10,
                PreambleLength::Four => 0b11,
            }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum IdErrorCodeTolerance {
    Bits0,
    #[default]
    Bits1,
    Bits2,
    Bits3,
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum PreabmelPatternDetectionLength {
    Bits0,
    Bits4,
    Bits8,
    #[default]
    Bits16,
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct Code2 {
    id_error_code_tolerance: IdErrorCodeTolerance,
    preamble_pattern_detection_length: PreabmelPatternDetectionLength,
}

impl Register for Code2 {
    fn id() -> u8 {
        0x20
    }
}

impl WritableRegister<u8> for Code2 {}

impl Into<u8> for Code2 {
    fn into(self) -> u8 {
        0 | match self.id_error_code_tolerance {
            IdErrorCodeTolerance::Bits0 => 0,
            IdErrorCodeTolerance::Bits1 => 0b0100,
            IdErrorCodeTolerance::Bits2 => 0b1000,
            IdErrorCodeTolerance::Bits3 => 0b1100,
        } | match self.preamble_pattern_detection_length {
            PreabmelPatternDetectionLength::Bits0 => 0,
            PreabmelPatternDetectionLength::Bits4 => 0b01,
            PreabmelPatternDetectionLength::Bits8 => 0b10,
            PreabmelPatternDetectionLength::Bits16 => 0b11,
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct Code3 {
    encryption_key: u8,
}

impl WritableRegister<u8> for Code3 {}

impl Register for Code3 {
    fn id() -> u8 {
        0x21
    }
}

impl Into<u8> for Code3 {
    fn into(self) -> u8 {
        self.encryption_key & 0b0111_1111
    }
}
