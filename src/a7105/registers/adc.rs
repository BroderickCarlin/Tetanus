use super::*;

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum RssiMargin {
    Five,
    Ten,
    Fifteen,
    #[default]
    Twenty,
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum AdcClockSpeed {
    #[default]
    Mhz4,
    Mhz8,
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum AdcCaptureMode {
    Single,
    #[default]
    Continuous,
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct AdcControl {
    pub margin: RssiMargin,
    /// If `true`, RSSI measurement will end when carrier detected and ID code word received
    pub rssi_end_early: bool,
    pub adc_clock_speed: AdcClockSpeed,
    pub capture_mode: AdcCaptureMode,
}

impl Register for AdcControl {
    fn id() -> u8 {
        0x1E
    }
}

impl WritableRegister<u8> for AdcControl {}

impl Into<u8> for AdcControl {
    fn into(self) -> u8 {
        0b0000_0010
            | match self.margin {
                RssiMargin::Five => 0b0000_0000,
                RssiMargin::Ten => 0b0100_0000,
                RssiMargin::Fifteen => 0b1000_0000,
                RssiMargin::Twenty => 0b1100_0000,
            }
            | if self.rssi_end_early { 0b0010_0000 } else { 0 }
            | match self.adc_clock_speed {
                AdcClockSpeed::Mhz4 => 0b0000_0000,
                AdcClockSpeed::Mhz8 => 0b0001_0000,
            }
            | match self.capture_mode {
                AdcCaptureMode::Single => 0b0000_0000,
                AdcCaptureMode::Continuous => 0b0000_0001,
            }
    }
}
