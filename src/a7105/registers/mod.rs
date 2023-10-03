pub use adc::*;
pub use calibration::*;
pub use cko::*;
pub use clock::*;
pub use code::*;
pub use data_rate::*;
pub use delay::*;
pub use fifo::*;
pub use gpio::*;
pub use id::*;
pub use if_calibration::*;
pub use mode::*;
pub use pll::*;
pub use rc_osc::*;
pub use rssi::*;
pub use rx::*;
pub use tx::*;

mod adc;
mod calibration;
mod cko;
mod clock;
mod code;
mod data_rate;
mod delay;
mod fifo;
mod gpio;
mod id;
mod if_calibration;
mod mode;
mod pll;
mod rc_osc;
mod rssi;
mod rx;
mod tx;

/// The generic top level trait for all register values
pub trait Register {
    fn id() -> u8;
}

/// A marker trait for registers that are readable
pub trait ReadableRegister<V>: Register + From<V> {}

/// A marker trait for registers that are writable
pub trait WritableRegister<V>: Register + Into<V> {}
