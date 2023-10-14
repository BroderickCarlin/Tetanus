pub enum Commands {
    /// A Strobe specific command for the A7105
    Strobe(Strobe),
    /// A software reset of the A7105
    Reset,
}

pub enum Strobe {
    /// Set the A7105 into sleep mode
    Sleep,
    /// Set the A7105 into Idle mode
    Idle,
    /// Set the A7105 into Standby mode
    Standby,
    /// Set the A7105 into PLL mode
    Pll,
    /// Set the A7105 into RX mode
    Rx,
    /// Set the A7105 into TX mode
    Tx,
    /// Reset A7105 FIFO write pointer.
    FifoWritePointerReset,
    /// Reset A7105 FIFO read pointer.
    FifoReadPointerReset,
}

impl Into<u8> for Strobe {
    fn into(self) -> u8 {
        match self {
            Strobe::Sleep => 0b1000_0000,
            Strobe::Idle => 0b1001_0000,
            Strobe::Standby => 0b1010_0000,
            Strobe::Pll => 0b1011_0000,
            Strobe::Rx => 0b1100_0000,
            Strobe::Tx => 0b1101_0000,
            Strobe::FifoWritePointerReset => 0b1110_0000,
            Strobe::FifoReadPointerReset => 0b1111_0000,
        }
    }
}
