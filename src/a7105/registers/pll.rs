use super::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Pll1 {
    // Not sure what this actually represents - this is probably wrong
    pub lo_channel: u8,
}

impl Default for Pll1 {
    fn default() -> Self {
        Self { lo_channel: 0 }
    }
}

impl Register for Pll1 {
    fn id() -> u8 {
        0x0F
    }
}

impl ReadableRegister<u8> for Pll1 {}
impl WritableRegister<u8> for Pll1 {}

impl From<u8> for Pll1 {
    fn from(lo_channel: u8) -> Self {
        Self { lo_channel }
    }
}

impl Into<u8> for Pll1 {
    fn into(self) -> u8 {
        self.lo_channel
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Pll2 {
    // Not sure what this actually represents - this is probably wrong
    pub crystal_freq_doubler: bool,
    pub rf_pll_ref_counter: u8,
    pub pll_chn_step: u8,
    pub ip8: bool,
}

impl Default for Pll2 {
    fn default() -> Self {
        Self {
            crystal_freq_doubler: true,
            rf_pll_ref_counter: 0,
            pll_chn_step: 0b1111,
            ip8: false,
        }
    }
}

impl Register for Pll2 {
    fn id() -> u8 {
        0x10
    }
}

impl ReadableRegister<u8> for Pll2 {}
impl WritableRegister<u8> for Pll2 {}

impl From<u8> for Pll2 {
    fn from(val: u8) -> Self {
        Self {
            crystal_freq_doubler: (val & 0b1000_0000) != 0,
            rf_pll_ref_counter: (val & 0b0110_0000) >> 5,
            pll_chn_step: (val & 0b0001_1110) >> 1,
            ip8: (val & 0b1) != 0,
        }
    }
}

impl Into<u8> for Pll2 {
    fn into(self) -> u8 {
        u8::from(self.crystal_freq_doubler) << 7
            | self.rf_pll_ref_counter.min(0b11) << 5
            | self.pll_chn_step.min(0b1111) << 1
            | u8::from(self.ip8)
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Pll3 {
    pub bip: u8,
}

impl Default for Pll3 {
    fn default() -> Self {
        Self { bip: 0x04 }
    }
}

impl Register for Pll3 {
    fn id() -> u8 {
        0x11
    }
}

impl ReadableRegister<u8> for Pll3 {}
impl WritableRegister<u8> for Pll3 {}

impl From<u8> for Pll3 {
    fn from(bip: u8) -> Self {
        Self { bip }
    }
}

impl Into<u8> for Pll3 {
    fn into(self) -> u8 {
        self.bip
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Pll4 {
    pub bfp: u8,
}

impl Default for Pll4 {
    fn default() -> Self {
        Self { bfp: 0x0 }
    }
}

impl Register for Pll4 {
    fn id() -> u8 {
        0x12
    }
}

impl ReadableRegister<u8> for Pll4 {}
impl WritableRegister<u8> for Pll4 {}

impl From<u8> for Pll4 {
    fn from(bfp: u8) -> Self {
        Self { bfp }
    }
}

impl Into<u8> for Pll4 {
    fn into(self) -> u8 {
        self.bfp
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Pll5 {
    pub bfp: u8,
}

impl Default for Pll5 {
    fn default() -> Self {
        Self { bfp: 0x0 }
    }
}

impl Register for Pll5 {
    fn id() -> u8 {
        0x12
    }
}

impl ReadableRegister<u8> for Pll5 {}
impl WritableRegister<u8> for Pll5 {}

impl From<u8> for Pll5 {
    fn from(bfp: u8) -> Self {
        Self { bfp }
    }
}

impl Into<u8> for Pll5 {
    fn into(self) -> u8 {
        self.bfp
    }
}