use super::*;

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum PinOutputMode {
    DckInTxRckInRx,
    FifoPointerFlag,
    Fsync,
    FsyncDiv2,
    FsyncDiv4,
    #[default]
    FsyncDiv8,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct CkoPinControl {
    pub external_clock_output_enabled: bool,
    pub pin_output_mode: PinOutputMode,
    pub invert_output: bool,
    pub output_enabled: bool,
    pub invert_spi_clk_input: bool,
}

impl Default for CkoPinControl {
    fn default() -> Self {
        Self {
            external_clock_output_enabled: true,
            pin_output_mode: Default::default(),
            invert_output: false,
            output_enabled: true,
            invert_spi_clk_input: false,
        }
    }
}

impl Register for CkoPinControl {
    fn id() -> u8 {
        0x06
    }
}

impl WritableRegister<u8> for CkoPinControl {}

impl Into<u8> for CkoPinControl {
    fn into(self) -> u8 {
        u8::from(self.external_clock_output_enabled) << 7
            | match self.pin_output_mode {
                PinOutputMode::DckInTxRckInRx => 0b0000_0000,
                PinOutputMode::FifoPointerFlag => 0b0001_0000,
                PinOutputMode::Fsync => 0b0010_0000,
                PinOutputMode::FsyncDiv2 => 0b0010_1000,
                PinOutputMode::FsyncDiv4 => 0b0011_0000,
                PinOutputMode::FsyncDiv8 => 0b0011_1000,
            }
            | u8::from(self.invert_output) << 2
            | u8::from(self.output_enabled) << 1
            | u8::from(self.invert_spi_clk_input)
    }
}
