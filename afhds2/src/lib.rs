#![no_std]

use a7105::{commands::Command, registers::*, A7105};

/// Magic ID for the a7105 for AFHDS2A flysky protocol
pub const RADIO_ID: u32 = 0x5475C52A;

pub struct Afhds2<SPI, P> {
    radio: A7105<SPI>,
    gpio: P,
}

impl<SPI, P> Afhds2<SPI, P> {
    /// Create a new instance of a [`Afhds2`]
    ///
    /// This method does not initialize the underlying radio hardware.
    pub const fn new(spi: SPI, gpio: P) -> Self {
        Self {
            radio: A7105::new(spi),
            gpio,
        }
    }

    /// Createa a new instance of a [`Afhds2`] from the provided [`A7105`]
    ///
    /// This method is useful for cases where you may want to initialize the underlying hardware
    /// manually, for example to configure the PLL, crystal, or GPIO pins.
    pub const fn from_radio(radio: A7105<SPI>, gpio: P) -> Self {
        Self { radio, gpio }
    }
}

#[cfg(feature = "blocking")]
impl<SPI, P> Afhds2<SPI, P>
where
    SPI: embedded_hal::spi::SpiDevice,
    P: embedded_hal::digital::InputPin,
{
    /// A function to configure the underlying radio hardware
    ///
    /// This method will reset the radio to its defaults, with the following modifications:
    ///
    /// | Config   | Value   | Description |
    /// |----------|---------|-------------|
    /// | GPIO1    | MISO    | Configures the GPIO1 pin to act as the MISO pin |
    /// | Radio ID | [`RADIO_ID`] | Configures the Radio ID |
    /// | Auto RSSI | True | Automatically perform RSSI measurement when entering RX mode
    /// | Data mode | FIFO | Use a FIFO for interfacing with the RX/TX data
    pub fn configure_radio<D>(&mut self, mut delay: D) -> Result<(), SPI::Error>
    where
        D: embedded_hal::delay::DelayUs,
    {
        // Start by resetting the radio
        self.radio.command(Command::Reset)?;

        // Give it time to actually perform the reset before continuing
        delay.delay_ms(50);

        // Configure GPIO1 as our MISO pin
        self.radio.write_reg(Gpio1PinControl {
            pin_function: GpioPinFunction::Sdo,
            output_enabled: true,
            ..Default::default()
        })?;

        // Set the radio ID
        self.radio.write_reg(IdData { id: RADIO_ID })?;

        // Mode Control
        self.radio.write_reg(ModeControl {
            auto_rssi: true,
            data_mode: DataMode::FIFO,
            ..Default::default()
        })?;

        // FIFO Reg. 1
        self.radio.write_reg(Fifo1 { end_pointer: 0x25 })?;

        // FIFO Reg. 2
        self.radio.write_reg(Fifo2 {
            margin: 0,
            segment: 0,
        })?;

        //     // GPIO2
        //     debug!("Setting up GPIO 2 pin...");
        //     // self.blocking_write_bytes(0xc, &[0b00_0001_01]);
        //     self.blocking_write_bytes(0xc, &[0b00_0000_01]);

        //     // RC OSC Register 1
        //     debug!("Setting RC OSC Register 1");
        //     self.blocking_write_bytes(0x7, &[0x00]);

        //     // RC OSC Register 2
        //     debug!("Setting RC OSC Register 2");
        //     self.blocking_write_bytes(0x8, &[0x00]);

        //     // Data Rate
        //     debug!("Setting up Data Rate");
        //     self.blocking_write_bytes(0xe, &[0x00]);

        // RC OSC register 3
        self.radio.write_reg(RcOsc3 {
            clock_select: ClockSelect::FSyncDiv8,
        })?;

        // Clock Register
        self.radio.write_reg(Clock {
            external_crystal_osc: true,
            clock_generation_ref_cnt: 0,
            ..Default::default()
        })?;

        // PLL Register 1
        self.radio.write_reg(Pll1 { channel: 0x50 })?;

        // PLL Register 5
        self.radio.write_reg(Pll5 { bfp: 2 })?;

        // PLL Register 2
        //     debug!("Setting PLL Register 2");
        //     self.blocking_write_bytes(0x10, &[0x9e]);

        //     // PLL Register 3
        //     debug!("Setting PLL Register 3");
        //     self.blocking_write_bytes(0x11, &[0x4b]);

        //     // PLL Register 4
        //     debug!("Setting PLL Register 4");
        //     self.blocking_write_bytes(0x12, &[0x00]);

        //     // TX Register 1
        //     debug!("Setting TX Register 1");
        //     self.blocking_write_bytes(0x14, &[0x16]);

        // TODO: might have to set this?
        //     // TX Register 2
        //     debug!("Setting TX Register 2");
        //     self.blocking_write_bytes(0x15, &[0x2b]);

        //     // Delay Register 1
        //     debug!("Setting Delay Register 1");
        //     self.blocking_write_bytes(0x16, &[0x12]);

        // Delay Register 2
        self.radio.write_reg(Delay2 {
            xtal_settling_delay: XtalSettlingDelay::Us200,
            agc_delay_settling: AgcDelaySettling::Us10,
            rssi_measurement_delay: RssiMeasurementDelay::Us10,
        })?;

        // Rx Register
        self.radio.write_reg(Rx {
            freq_compensation_enable: true,
            ..Default::default()
        })?;

        // Rx Gain Register 1
        self.radio.write_reg(RxGain1 {
            manual_vga_calibration: true,
            ..Default::default()
        })?;

        //     // Rx Gain Register 4
        //     debug!("Setting Rx Gain Register");
        //     self.blocking_write_bytes(0x1c, &[0x2a]);

        // RSSI Threshold
        self.radio
            .write_reg(RssiCarrierDetectThreshold { threshold: 0x32 })?;

        // Code Register 1
        self.radio.write_reg(Code1 {
            fec_enabled: true,
            crc_enabled: true,
            id_length: IdLength::Four,
            preable_length: PreambleLength::Four,
            ..Default::default()
        })?;

        // Code Register 2
        self.radio.write_reg(Code2 {
            demodulator_dc_estimation_average_mode: 1,
            id_error_code_tolerance: IdErrorCodeTolerance::Bits0,
            preamble_pattern_detection_length: PreabmelPatternDetectionLength::Bits16,
            ..Default::default()
        })?;

        // Code Register 3
        // Why do we set the encryption key?!
        self.radio.write_reg(Code3 {
            encryption_key: 0b1100_0011,
        })?;

        //     // IF Calibration Register 1
        //     debug!("Setting IF Calibration Register 1");
        //     self.blocking_write_bytes(0x22, &[0x00]);

        // VCO current Calibration Register
        self.radio.write_reg(VcoCurrentCalibration::Automatic)?;

        //     // VCO Single band Calibration Register 1
        //     debug!("Setting VCO Single band Calibration Register 1");
        //     self.blocking_write_bytes(0x25, &[0x00]);

        //     // VCO Single band Calibration Register 2
        //     debug!("Setting VCO Single band Calibration Register 2");
        //     self.blocking_write_bytes(0x26, &[0x3b]);

        //     // Battery detect Register
        //     debug!("Setting Battery detect Register");
        //     self.blocking_write_bytes(0x27, &[0x00]);

        // BRODERICK: need to implement

        //     // TX Test Register
        //     debug!("Setting TX Test Register");
        //     self.blocking_write_bytes(0x28, &[0x17]);

        //     // Rx DEM test Register 1
        //     debug!("Setting Rx DEM test Register 1");
        //     self.blocking_write_bytes(0x29, &[0x47]);

        //     // Rx DEM test Register 2
        //     debug!("Setting Rx DEM test Register 2");
        //     self.blocking_write_bytes(0x2a, &[0x80]);

        //     // Charge Pump
        //     debug!("Setting Charge Pump Current Register");
        //     self.blocking_write_bytes(0x2b, &[0x03]);

        //     // Crystal Test
        //     debug!("Setting Crystal Test Register");
        //     self.blocking_write_bytes(0x2c, &[0x01]);

        //     // PLL Test
        //     debug!("Setting PLL Test Register");
        //     self.blocking_write_bytes(0x2d, &[0x45]);

        //     // VCO Test 1
        //     debug!("Setting VCO Test Register 1");
        //     self.blocking_write_bytes(0x2e, &[0x18]);

        //     // VCO Test 2
        //     debug!("Setting VCO Test Register 2");
        //     self.blocking_write_bytes(0x2f, &[0x00]);

        //     // IFAT Register
        //     debug!("Setting IFAT Register");
        //     self.blocking_write_bytes(0x30, &[0x01]);

        //     // RScale Register
        //     debug!("Setting RScale Register");
        //     self.blocking_write_bytes(0x31, &[0x0f]);

        //     // Set to Standby mode
        //     self.blocking_strobe(Strobe::Standby);

        //     // Calibrate IF filter bank
        //     debug!("IF filter bank calibration...");
        //     self.blocking_write_bytes(0x02, &[0x01]);
        //     self.blocking_wait_auto_clear(0x02, 0x01);

        //     debug!("VCO Current Calibration");
        //     //Recomended calibration from A7105 Datasheet
        //     self.blocking_write_bytes(0x24, &[0x13]);

        //     debug!("VCO bank calibration");
        //     self.blocking_write_bytes(0x26, &[0x3b]);

        //     debug!("VCO Bank Calibrate channel 0");
        //     self.blocking_write_bytes(0x0f, &[0]); // set channel
        //     self.blocking_write_bytes(0x02, &[0x02]);
        //     self.blocking_wait_auto_clear(0x02, 0x02);

        //     let cal0 = &mut [0];
        //     self.blocking_read_register(0x25, cal0);
        //     if (cal0[0] & 0x08) != 0 {
        //         warn!("!!! VCO Calibration fail");
        //     }

        //     debug!("VCO Bank Calibrate channel 0xa0");
        //     self.blocking_write_bytes(0x0f, &[0xa0]); // set channel
        //     self.blocking_write_bytes(0x02, &[0x02]);
        //     self.blocking_wait_auto_clear(0x02, 0x02);

        //     let cal0 = &mut [0];
        //     self.blocking_read_register(0x25, cal0);
        //     debug!("vco cal a0={:02x}", cal0[0]);
        //     if (cal0[0] & 0x08) != 0 {
        //         debug!("!!! VCO Calibration fail");
        //     }

        //     debug!("Reset VCO band calibration");
        //     self.blocking_write_bytes(0x25, &[0x08]);

        //     debug!("Strobe standby");
        //     self.blocking_strobe(Strobe::Standby);
        //     self.register_dump();

        //     let stored_channel = &mut [0];
        //     self.blocking_read_register(0x0f, stored_channel);
        //     if stored_channel[0] != 0xa0 {
        //         warn!(
        //             "Stored channel should have been 0xa0, was {}",
        //             stored_channel[0]
        //         );
        //     }
        //     debug!("End of radio_init");
        // }

        Ok(())
    }
}

// impl<'spi, 'cs, T, C, G, RD, TD> A7105<'spi, 'cs, T, C, G, RD, TD>
// where
//     T: Instance,
//     C: Pin,
//     G: Pin,
//     RD: RxDma<T>,
//     TD: TxDma<T>,
// {
//     pub fn new<Sck, Mosi, Miso, Cs, Gp, TxDma, RxDma>(
//         peripheral: T,
//         sck: Sck,
//         mosi: Mosi,
//         miso: Miso,
//         cs: Cs,
//         tx_dma: TxDma,
//         rx_dma: RxDma,
//         gpio1: Gp,
//     ) -> Self
//     where
//         Sck: Peripheral + 'spi,
//         <Sck as Peripheral>::P: SckPin<T>,
//         Mosi: Peripheral + 'spi,
//         <Mosi as Peripheral>::P: MosiPin<T>,
//         Miso: Peripheral + 'spi,
//         <Miso as Peripheral>::P: MisoPin<T>,
//         Cs: Peripheral<P = C> + 'cs,
//         Gp: Peripheral<P = G> + 'cs,
//         TxDma: Peripheral<P = TD> + 'spi,
//         RxDma: Peripheral<P = RD> + 'spi,
//     {
//         let mut spi_config = Config::default();
//         // spi_config.frequency = Hertz(10_000_000);

//         Self {
//             spi: Spi::new(peripheral, sck, mosi, miso, tx_dma, rx_dma, spi_config),
//             cs: Output::new(cs, Level::High, Speed::VeryHigh),
//             gpio1: Input::new(gpio1, Pull::None),
//         }
//     }

// pub async fn init(&mut self) {
//     // Wait for it to be ready, in case we just powered on.
//     Timer::after(Duration::from_millis(25)).await;

//     // Zero - reset register.
//     debug!("Resetting the a7105");
//     self.blocking_write_bytes(0x0, &[0x00]);
//     Timer::after(Duration::from_millis(50)).await;

//     // Configure GPIO1 as our MISO pin
//     debug!("Configuring GPIO 1 Pin...");
//     self.blocking_write_bytes(0xB, &[0b00_0110_01]);

//     // Write the Radio ID
//     debug!("Setting Radio ID to {:?}", RADIO_ID);
//     self.blocking_write_bytes(0x6, RADIO_ID);

//     // Mode Control
//     debug!("Setting Mode Control");
//     self.blocking_write_bytes(0x1, &[0b0100_0010]);

//     // Calibration Control
//     debug!("Setting Calibration Control");
//     self.blocking_write_bytes(0x2, &[0]);

//     // FIFO Reg. 1
//     debug!("Setting FIFO Reg. 1");
//     self.blocking_write_bytes(0x3, &[0x25]);

//     // FIFO Reg. 2
//     debug!("Setting FIFO Reg. 2");
//     self.blocking_write_bytes(0x4, &[0x00]);

//     // RC OSC Register 1
//     debug!("Setting RC OSC Register 1");
//     self.blocking_write_bytes(0x7, &[0x00]);

//     // RC OSC Register 2
//     debug!("Setting RC OSC Register 2");
//     self.blocking_write_bytes(0x8, &[0x00]);

//     // RC OSC Register 3
//     debug!("Setting RC OSC Register 3");
//     self.blocking_write_bytes(0x9, &[0x00]);

//     // GPIO2
//     debug!("Setting up GPIO 2 pin...");
//     // self.blocking_write_bytes(0xc, &[0b00_0001_01]);
//     self.blocking_write_bytes(0xc, &[0b00_0000_01]);

//     // Clock Register
//     debug!("Setting up Clock");
//     self.blocking_write_bytes(0xd, &[0x5]);

//     // Data Rate
//     debug!("Setting up Data Rate");
//     self.blocking_write_bytes(0xe, &[0x00]);

//     // PLL Register 1
//     debug!("Setting PLL Register 1");
//     self.blocking_write_bytes(0xf, &[0x50]);

//     // PLL Register 2
//     debug!("Setting PLL Register 2");
//     self.blocking_write_bytes(0x10, &[0x9e]);

//     // PLL Register 3
//     debug!("Setting PLL Register 3");
//     self.blocking_write_bytes(0x11, &[0x4b]);

//     // PLL Register 4
//     debug!("Setting PLL Register 4");
//     self.blocking_write_bytes(0x12, &[0x00]);

//     // PLL Register 5
//     debug!("Setting PLL Register 5");
//     self.blocking_write_bytes(0x13, &[0x02]);

//     // TX Register 1
//     debug!("Setting TX Register 1");
//     self.blocking_write_bytes(0x14, &[0x16]);

//     // TX Register 2
//     debug!("Setting TX Register 2");
//     self.blocking_write_bytes(0x15, &[0x2b]);

//     // Delay Register 1
//     debug!("Setting Delay Register 1");
//     self.blocking_write_bytes(0x16, &[0x12]);

//     // Delay Register 2
//     debug!("Setting Delay Register 2");
//     self.blocking_write_bytes(0x17, &[0x00]);

//     // Rx Register
//     debug!("Setting Rx Register");
//     self.blocking_write_bytes(0x18, &[0x62]);

//     // Rx Gain Register 1
//     debug!("Setting Rx Gain Register");
//     self.blocking_write_bytes(0x19, &[0x80]);

//     // Rx Gain Register 4
//     debug!("Setting Rx Gain Register");
//     self.blocking_write_bytes(0x1c, &[0x2a]);

//     // RSSI Threshold
//     debug!("Setting RSSI Threshold Register");
//     self.blocking_write_bytes(0x1d, &[0x32]);

//     // ADC Control
//     debug!("Setting ADC Control Register");
//     self.blocking_write_bytes(0x1e, &[0xc3]);

//     // Code Register 1
//     debug!("Setting Code Register 1");
//     self.blocking_write_bytes(0x1f, &[0b0001_1111]);

//     // Code Register 2
//     debug!("Setting Code Register 2");
//     self.blocking_write_bytes(0x20, &[0x13]);

//     // Code Register 3
//     debug!("Setting Code Register 3");
//     self.blocking_write_bytes(0x21, &[0xc3]);

//     // IF Calibration Register 1
//     debug!("Setting IF Calibration Register 1");
//     self.blocking_write_bytes(0x22, &[0x00]);

//     // VCO current Calibration Register
//     debug!("Setting VCO current Calibration Register");
//     self.blocking_write_bytes(0x24, &[0x00]);

//     // VCO Single band Calibration Register 1
//     debug!("Setting VCO Single band Calibration Register 1");
//     self.blocking_write_bytes(0x25, &[0x00]);

//     // VCO Single band Calibration Register 2
//     debug!("Setting VCO Single band Calibration Register 2");
//     self.blocking_write_bytes(0x26, &[0x3b]);

//     // Battery detect Register
//     debug!("Setting Battery detect Register");
//     self.blocking_write_bytes(0x27, &[0x00]);

//     // TX Test Register
//     debug!("Setting TX Test Register");
//     self.blocking_write_bytes(0x28, &[0x17]);

//     // Rx DEM test Register 1
//     debug!("Setting Rx DEM test Register 1");
//     self.blocking_write_bytes(0x29, &[0x47]);

//     // Rx DEM test Register 2
//     debug!("Setting Rx DEM test Register 2");
//     self.blocking_write_bytes(0x2a, &[0x80]);

//     // Charge Pump
//     debug!("Setting Charge Pump Current Register");
//     self.blocking_write_bytes(0x2b, &[0x03]);

//     // Crystal Test
//     debug!("Setting Crystal Test Register");
//     self.blocking_write_bytes(0x2c, &[0x01]);

//     // PLL Test
//     debug!("Setting PLL Test Register");
//     self.blocking_write_bytes(0x2d, &[0x45]);

//     // VCO Test 1
//     debug!("Setting VCO Test Register 1");
//     self.blocking_write_bytes(0x2e, &[0x18]);

//     // VCO Test 2
//     debug!("Setting VCO Test Register 2");
//     self.blocking_write_bytes(0x2f, &[0x00]);

//     // IFAT Register
//     debug!("Setting IFAT Register");
//     self.blocking_write_bytes(0x30, &[0x01]);

//     // RScale Register
//     debug!("Setting RScale Register");
//     self.blocking_write_bytes(0x31, &[0x0f]);

//     // Set to Standby mode
//     self.blocking_strobe(Strobe::Standby);

//     // Calibrate IF filter bank
//     debug!("IF filter bank calibration...");
//     self.blocking_write_bytes(0x02, &[0x01]);
//     self.blocking_wait_auto_clear(0x02, 0x01);

//     debug!("VCO Current Calibration");
//     //Recomended calibration from A7105 Datasheet
//     self.blocking_write_bytes(0x24, &[0x13]);

//     debug!("VCO bank calibration");
//     self.blocking_write_bytes(0x26, &[0x3b]);

//     debug!("VCO Bank Calibrate channel 0");
//     self.blocking_write_bytes(0x0f, &[0]); // set channel
//     self.blocking_write_bytes(0x02, &[0x02]);
//     self.blocking_wait_auto_clear(0x02, 0x02);

//     let cal0 = &mut [0];
//     self.blocking_read_register(0x25, cal0);
//     if (cal0[0] & 0x08) != 0 {
//         warn!("!!! VCO Calibration fail");
//     }

//     debug!("VCO Bank Calibrate channel 0xa0");
//     self.blocking_write_bytes(0x0f, &[0xa0]); // set channel
//     self.blocking_write_bytes(0x02, &[0x02]);
//     self.blocking_wait_auto_clear(0x02, 0x02);

//     let cal0 = &mut [0];
//     self.blocking_read_register(0x25, cal0);
//     debug!("vco cal a0={:02x}", cal0[0]);
//     if (cal0[0] & 0x08) != 0 {
//         debug!("!!! VCO Calibration fail");
//     }

//     debug!("Reset VCO band calibration");
//     self.blocking_write_bytes(0x25, &[0x08]);

//     debug!("Strobe standby");
//     self.blocking_strobe(Strobe::Standby);
//     self.register_dump();

//     let stored_channel = &mut [0];
//     self.blocking_read_register(0x0f, stored_channel);
//     if stored_channel[0] != 0xa0 {
//         warn!(
//             "Stored channel should have been 0xa0, was {}",
//             stored_channel[0]
//         );
//     }
//     debug!("End of radio_init");
// }

// pub fn set_channel(&mut self, chn: u8) {
//     self.blocking_write_bytes(0x0f, &[chn]);
// }

// pub async fn wait_gpio(&mut self) -> bool {
//     let mut cnt = 0;
//     // Wait for it to go high
//     while self.gpio1.is_low() {
//         Timer::after(Duration::from_micros(100)).await;
//         cnt += 1;

//         if cnt > 2000 {
//             return false;
//         }
//     }

//     let mut cnt = 0;
//     // Wait for it to go low
//     while self.gpio1.is_high() {
//         Timer::after(Duration::from_micros(1)).await;
//         cnt += 1;

//         if cnt > 2000 {
//             return false;
//         }
//     }

//     true
// }

// fn register_dump(&mut self) {
//     let id = &mut [0, 0, 0, 0];
//     self.blocking_read_register(0x06, id);
//     debug!("Dumped ID: {:?}", id);
// }

// pub fn blocking_strobe(&mut self, cmd: Strobe) {
//     let byte: u8 = cmd.into();

//     self.cs.set_low();
//     self.spi.blocking_write(&[byte]).ok();
//     self.cs.set_high();
// }

// pub fn blocking_write_bytes(&mut self, mut addr: u8, bytes: &[u8]) {
//     // Mask off the bits indicating it is a write
//     addr &= 0x3f;

//     self.cs.set_low();
//     self.spi.blocking_write(&[addr]).ok();
//     self.spi.blocking_write(bytes).ok();
//     self.cs.set_high();
// }

// pub fn blocking_read_register(&mut self, mut addr: u8, bytes: &mut [u8]) {
//     // Mask off the bits indicating it is a read
//     addr = (addr & 0x3f) | 0x40;

//     self.cs.set_low();
//     self.spi.blocking_write(&[addr]).ok();
//     self.spi.blocking_read(bytes).ok();
//     self.cs.set_high();
// }

// pub fn blocking_wait_auto_clear(&mut self, addr: u8, bit: u8) {
//     let byte = &mut [0];
//     loop {
//         self.blocking_read_register(addr, byte);
//         if (byte[0] & bit) == 0 {
//             break;
//         }
//     }
// }

// pub fn blocking_strobe_then_read(&mut self, cmd: Strobe, mut addr: u8, bytes: &mut [u8]) {
//     let byte: u8 = cmd.into();

//     // Mask off the bits indicating it is a read
//     addr = (addr & 0x3f) | 0x40;

//     self.cs.set_low();
//     self.spi.blocking_write(&[byte, addr]).ok();
//     self.spi.blocking_read(bytes).ok();
//     self.cs.set_high();
// }

// pub fn blocking_write_then_strobe(&mut self, cmd: Strobe, mut addr: u8, bytes: &[u8]) {
//     let byte: u8 = cmd.into();

//     // Mask off the bits indicating it is a read
//     addr &= 0x3f;

//     self.cs.set_low();
//     self.spi.blocking_write(&[addr]).ok();
//     self.spi.blocking_write(bytes).ok();
//     self.spi.blocking_write(&[byte]).ok();
//     self.cs.set_high();
// }
// }
