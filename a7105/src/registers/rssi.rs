use super::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RssiCarrierDetectThreshold {
    pub threshold: u8,
}

impl Default for RssiCarrierDetectThreshold {
    fn default() -> Self {
        Self {
            threshold: 0b1001_0001,
        }
    }
}

impl Register for RssiCarrierDetectThreshold {
    fn id() -> u8 {
        0x1D
    }
}

impl WritableRegister<u8> for RssiCarrierDetectThreshold {}

impl Into<u8> for RssiCarrierDetectThreshold {
    fn into(self) -> u8 {
        self.threshold
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct RssiAdcOutput {
    pub voltage: f32,
}

impl Register for RssiAdcOutput {
    fn id() -> u8 {
        0x1D
    }
}

impl ReadableRegister<u8> for RssiAdcOutput {}

impl From<u8> for RssiAdcOutput {
    fn from(val: u8) -> Self {
        Self {
            voltage: val as f32 * 1.2 / 256.,
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_rx_register() {
        let default: u8 = RssiCarrierDetectThreshold::default().into();
        assert_eq!(default, 0b1001_0001);

        assert_eq!(RssiCarrierDetectThreshold::id(), 0x1D);
    }
}
