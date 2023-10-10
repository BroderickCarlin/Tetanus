use super::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum VcoCurrentCalibration {
    Automatic,
    Manual(u8)
}

impl Default for VcoCurrentCalibration {
    fn default() -> Self {
        Self::Manual(0b011)
    }
}

impl Register for VcoCurrentCalibration {
    fn id() -> u8 {
        0x24
    }
}

impl WritableRegister<u8> for VcoCurrentCalibration {}

impl Into<u8> for VcoCurrentCalibration {
    fn into(self) -> u8 {
        match self {
            Self::Automatic => 0,
            Self::Manual(val) => (val & 0b1111) | 0b1_0000
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct  VcoCurrentCalibrationResult {
    success: bool,
    value: u8
}

impl Register for VcoCurrentCalibrationResult {
    fn id() -> u8 {
        0x24
    }
}

impl ReadableRegister<u8> for VcoCurrentCalibrationResult {}

impl From<u8> for VcoCurrentCalibrationResult {
    fn from(val: u8) -> Self {
        Self {
            success: (val & 0b1_0000) != 0, 
            value: val & 0b1111
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum VcoSingleBandCalibration1 {
    #[default]
    Automatic,
    Manual(u8)
}

impl Register for VcoSingleBandCalibration1 {
    fn id() -> u8 {
        0x25
    }
}

impl WritableRegister<u8> for VcoSingleBandCalibration1 {}

impl Into<u8> for VcoSingleBandCalibration1 {
    fn into(self) -> u8 {
        match self {
            Self::Automatic => 0,
            Self::Manual(val) => (val & 0b111) | 0b1000
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum VcoVoltageOutput {
    /// VT<VTL<VTH
    VtMin,
    /// VTL<VT<VTH
    VtMid,
    /// VTL<VTH<VT
    VtMax
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct  VcoSingleBandCalibration1Result {
    voltage_output: VcoVoltageOutput,
    success: bool,
    value: u8
}

impl Register for VcoSingleBandCalibration1Result {
    fn id() -> u8 {
        0x25
    }
}

impl ReadableRegister<u8> for VcoSingleBandCalibration1Result {}

impl From<u8> for VcoSingleBandCalibration1Result {
    fn from(val: u8) -> Self {
        Self {
            voltage_output: match (val & 0b11_0000) >> 4 {
                0b00 => VcoVoltageOutput::VtMin,
                0b01 => VcoVoltageOutput::VtMid,
                _ => VcoVoltageOutput::VtMax
            },
            success: (val & 0b1000) != 0, 
            value: val & 0b111
        }
    }
}


#[derive(PartialEq, Debug, Copy, Clone)]
pub struct VcoSingleBandCalibration2 {
    voltage_upper_threshold: u8
}