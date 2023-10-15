use super::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum GpioPinFunction {
    /// Wait until TX or RX finished
    Wtr,
    /// (TX) end of access code / (RX) Frame Sync
    EoacOrFsync,
    /// (TX) TX modulation enable / (RX) Carrier Detect
    TmeoOrCd,
    PreableDetectOutput,
    Default,
    InPhaseDemodulatorInput,
    Sdo,
    Trxd,
    Rxd,
    Txd,
    InPhaseDemodulatorExternalInput,
    ExternalFsyncInput,
}

impl Into<u8> for GpioPinFunction {
    fn into(self) -> u8 {
        match self {
            GpioPinFunction::Wtr => 0b0000_0000,
            GpioPinFunction::EoacOrFsync => 0b0000_0100,
            GpioPinFunction::TmeoOrCd => 0b0000_1000,
            GpioPinFunction::PreableDetectOutput => 0b0000_1100,
            GpioPinFunction::Default => 0b0001_0000,
            GpioPinFunction::InPhaseDemodulatorInput => 0b0001_0100,
            GpioPinFunction::Sdo => 0b0001_1000,
            GpioPinFunction::Trxd => 0b0011_1000,
            GpioPinFunction::Rxd => 0b0010_0000,
            GpioPinFunction::Txd => 0b0010_0100,
            GpioPinFunction::InPhaseDemodulatorExternalInput => 0b0010_1000,
            GpioPinFunction::ExternalFsyncInput => 0b0010_1100,
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Gpio1PinControl {
    pub pin_function: GpioPinFunction,
    pub invert_output: bool,
    pub output_enabled: bool,
}

impl Default for Gpio1PinControl {
    fn default() -> Self {
        Self {
            pin_function: GpioPinFunction::Wtr,
            invert_output: false,
            output_enabled: true,
        }
    }
}

impl Register for Gpio1PinControl {
    fn id() -> u8 {
        0x0B
    }
}

impl WritableRegister<u8> for Gpio1PinControl {}

impl Into<u8> for Gpio1PinControl {
    fn into(self) -> u8 {
        Into::<u8>::into(self.pin_function)
            | u8::from(self.invert_output) << 1
            | u8::from(self.output_enabled)
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Gpio2PinControl {
    pub pin_function: GpioPinFunction,
    pub invert_output: bool,
    pub output_enabled: bool,
}

impl Default for Gpio2PinControl {
    fn default() -> Self {
        Self {
            pin_function: GpioPinFunction::Default,
            invert_output: false,
            output_enabled: true,
        }
    }
}

impl Register for Gpio2PinControl {
    fn id() -> u8 {
        0x0C
    }
}

impl WritableRegister<u8> for Gpio2PinControl {}

impl Into<u8> for Gpio2PinControl {
    fn into(self) -> u8 {
        Into::<u8>::into(self.pin_function)
            | u8::from(self.invert_output) << 1
            | u8::from(self.output_enabled)
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_gpio1_pin_control() {
        let default: u8 = Gpio1PinControl::default().into();
        assert_eq!(default, 0b1);

        assert_eq!(Gpio1PinControl::id(), 0xB);
    }

    #[test]
    fn test_gpio2_pin_control() {
        let default: u8 = Gpio2PinControl::default().into();
        assert_eq!(default, 0b10001);

        assert_eq!(Gpio2PinControl::id(), 0xC);
    }
}
