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
