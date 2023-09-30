use super::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Bandwidth {
    khz250,
    khz500,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Rx {
    pub freq_compensation_enable: bool,
    pub data_invert: bool,
    pub bandwidth: Bandwidth,
    pub lowside_band_select: bool,
}

impl Default for Rx {
    fn default() -> Self {
        Self {
            freq_compensation_enable: false,
            data_invert: false,
            bandwidth: Bandwidth::khz500,
            lowside_band_select: true,
        }
    }
}

impl Register for Rx {
    fn id() -> u8 {
        0x18
    }
}

impl WritableRegister<u8> for Rx {}

impl Into<u8> for Rx {
    fn into(self) -> u8 {
        0b0110_0000
            | u8::from(self.freq_compensation_enable) << 4
            | u8::from(self.data_invert) << 3
            | match self.bandwidth {
                Bandwidth::khz250 => 0b1,
                Bandwidth::khz500 => 0b0,
            } << 1
            | u8::from(self.lowside_band_select)
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum MixerGain {
    db24,
    db18,
    db12,
    db6,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum LnaGain {
    db24,
    db18,
    db12,
    db6,
    db0,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RxGain1 {
    pub manual_vga_calibration: bool,
    pub mixer_gain: MixerGain,
    pub lna_gain: LnaGain,
}

impl Default for RxGain1 {
    fn default() -> Self {
        Self {
            manual_vga_calibration: false,
            mixer_gain: MixerGain::db24,
            lna_gain: LnaGain::db24,
        }
    }
}

impl Register for RxGain1 {
    fn id() -> u8 {
        0x19
    }
}

impl WritableRegister<u8> for RxGain1 {}
impl ReadableRegister<u8> for RxGain1 {}

impl Into<u8> for RxGain1 {
    fn into(self) -> u8 {
        u8::from(self.manual_vga_calibration) << 7
            | match self.mixer_gain {
                MixerGain::db24 => 0b00,
                MixerGain::db18 => 0b01,
                MixerGain::db12 => 0b10,
                MixerGain::db6 => 0b11,
            } << 3
            | match self.lna_gain {
                LnaGain::db24 => 0b000,
                LnaGain::db18 => 0b001,
                LnaGain::db12 => 0b010,
                LnaGain::db6 => 0b011,
                LnaGain::db0 => 0b100,
            }
    }
}

impl From<u8> for RxGain1 {
    fn from(val: u8) -> Self {
        Self {
            manual_vga_calibration: (val & 0b1000_0000) != 0,
            mixer_gain: match (val >> 3) & 0b11 {
                0b00 => MixerGain::db24,
                0b01 => MixerGain::db18,
                0b10 => MixerGain::db12,
                0b11 => MixerGain::db6,
                _ => unreachable!(),
            },
            lna_gain: match val & 0b111 {
                0b000 => LnaGain::db24,
                0b001 => LnaGain::db18,
                0b010 => LnaGain::db12,
                0b011 => LnaGain::db6,
                _ => LnaGain::db0,
            },
        }
    }
}
