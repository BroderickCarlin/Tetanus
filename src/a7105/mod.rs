use commands::Strobe;
use defmt::{debug, warn};
use embassy_stm32::{
    gpio::{Level, Output, Pin, Speed},
    spi::{Config, Instance, MisoPin, MosiPin, RxDma, SckPin, Spi, TxDma},
    time::Hertz,
    Peripheral,
};
use embassy_time::{Duration, Timer};

pub mod commands;

// Magic ID for the a7105 for AFHDS2A flysky protocol
const RADIO_ID: &[u8] = &[0x54, 0x75, 0xc5, 0x2a];

pub struct A7105<'spi, 'cs, T: Instance, C: Pin, RD, TD> {
    spi: Spi<'spi, T, TD, RD>,
    cs: Output<'cs, C>,
}

impl<'spi, 'cs, T, C, RD, TD> A7105<'spi, 'cs, T, C, RD, TD>
where
    T: Instance,
    C: Pin,
    RD: RxDma<T>,
    TD: TxDma<T>,
{
    pub fn new<Sck, Mosi, Miso, Cs, TxDma, RxDma>(
        peripheral: T,
        sck: Sck,
        mosi: Mosi,
        miso: Miso,
        cs: Cs,
        tx_dma: TxDma,
        rx_dma: RxDma,
    ) -> Self
    where
        Sck: Peripheral + 'spi,
        <Sck as Peripheral>::P: SckPin<T>,
        Mosi: Peripheral + 'spi,
        <Mosi as Peripheral>::P: MosiPin<T>,
        Miso: Peripheral + 'spi,
        <Miso as Peripheral>::P: MisoPin<T>,
        Cs: Peripheral<P = C> + 'cs,
        TxDma: Peripheral<P = TD> + 'spi,
        RxDma: Peripheral<P = RD> + 'spi,
    {
        let mut spi_config = Config::default();
        spi_config.frequency = Hertz(10_000_000);

        Self {
            spi: Spi::new(peripheral, sck, mosi, miso, tx_dma, rx_dma, spi_config),
            cs: Output::new(cs, Level::High, Speed::VeryHigh),
        }
    }

    pub async fn init(&mut self) {
        // Wait for it to be ready, in case we just powered on.
        Timer::after(Duration::from_millis(25)).await;

        // Zero - reset register.
        debug!("Resetting the a7105");
        self.blocking_write_bytes(0x0, &[0x00]);
        Timer::after(Duration::from_millis(50)).await;

        // Configure GPIO1 as our MISO pin
        debug!("Configuring GPIO 1 Pin...");
        self.blocking_write_bytes(0xB, &[0b00_0110_01]);

        // Write the Radio ID
        debug!("Setting Radio ID to {:?}", RADIO_ID);
        self.blocking_write_bytes(0x6, RADIO_ID);

        // Mode Control
        debug!("Setting Mode Control");
        self.blocking_write_bytes(0x1, &[0x42]);

        // Calibration Control
        debug!("Setting Calibration Control");
        self.blocking_write_bytes(0x2, &[0]);

        // FIFO Reg. 1
        debug!("Setting FIFO Reg. 1");
        self.blocking_write_bytes(0x3, &[0x25]);

        // FIFO Reg. 2
        debug!("Setting FIFO Reg. 2");
        self.blocking_write_bytes(0x4, &[0x00]);

        // RC OSC Register 1
        debug!("Setting RC OSC Register 1");
        self.blocking_write_bytes(0x7, &[0x00]);

        // RC OSC Register 2
        debug!("Setting RC OSC Register 2");
        self.blocking_write_bytes(0x8, &[0x00]);

        // RC OSC Register 3
        debug!("Setting RC OSC Register 3");
        self.blocking_write_bytes(0x9, &[0x00]);

        // GPIO2
        debug!("Setting up GPIO 2 pin...");
        self.blocking_write_bytes(0xc, &[0b00_0001_01]);

        // Clock Register
        debug!("Setting up Clock");
        self.blocking_write_bytes(0xd, &[0x5]);

        // Data Rate
        debug!("Setting up Data Rate");
        self.blocking_write_bytes(0xe, &[0x00]);

        // PLL Register 1
        debug!("Setting PLL Register 1");
        self.blocking_write_bytes(0xf, &[0x50]);

        // PLL Register 2
        debug!("Setting PLL Register 2");
        self.blocking_write_bytes(0x10, &[0x9e]);

        // PLL Register 3
        debug!("Setting PLL Register 3");
        self.blocking_write_bytes(0x11, &[0x4b]);

        // PLL Register 4
        debug!("Setting PLL Register 4");
        self.blocking_write_bytes(0x12, &[0x00]);

        // PLL Register 5
        debug!("Setting PLL Register 5");
        self.blocking_write_bytes(0x13, &[0x02]);

        // TX Register 1
        debug!("Setting TX Register 1");
        self.blocking_write_bytes(0x14, &[0x16]);

        // TX Register 2
        debug!("Setting TX Register 2");
        self.blocking_write_bytes(0x15, &[0x2b]);

        // Delay Register 1
        debug!("Setting Delay Register 1");
        self.blocking_write_bytes(0x16, &[0x12]);

        // Delay Register 2
        debug!("Setting Delay Register 2");
        self.blocking_write_bytes(0x17, &[0x00]);

        // Rx Register
        debug!("Setting Rx Register");
        self.blocking_write_bytes(0x18, &[0x62]);

        // Rx Gain Register 1
        debug!("Setting Rx Gain Register");
        self.blocking_write_bytes(0x19, &[0x80]);

        // Rx Gain Register 4
        debug!("Setting Rx Gain Register");
        self.blocking_write_bytes(0x1c, &[0x2a]);

        // RSSI Threshold
        debug!("Setting RSSI Threshold Register");
        self.blocking_write_bytes(0x1d, &[0x32]);

        // ADC Control
        debug!("Setting ADC Control Register");
        self.blocking_write_bytes(0x1e, &[0xc3]);

        // Code Register 1
        debug!("Setting Code Register 1");
        self.blocking_write_bytes(0x1f, &[0x1f]);

        // Code Register 2
        debug!("Setting Code Register 2");
        self.blocking_write_bytes(0x20, &[0x13]);

        // Code Register 3
        debug!("Setting Code Register 3");
        self.blocking_write_bytes(0x21, &[0xc3]);

        // IF Calibration Register 1
        debug!("Setting IF Calibration Register 1");
        self.blocking_write_bytes(0x22, &[0x00]);

        // VCO current Calibration Register
        debug!("Setting VCO current Calibration Register");
        self.blocking_write_bytes(0x24, &[0x00]);

        // VCO Single band Calibration Register 1
        debug!("Setting VCO Single band Calibration Register 1");
        self.blocking_write_bytes(0x25, &[0x00]);

        // VCO Single band Calibration Register 2
        debug!("Setting VCO Single band Calibration Register 2");
        self.blocking_write_bytes(0x26, &[0x3b]);

        // Battery detect Register
        debug!("Setting Battery detect Register");
        self.blocking_write_bytes(0x27, &[0x00]);

        // TX Test Register
        debug!("Setting TX Test Register");
        self.blocking_write_bytes(0x28, &[0x17]);

        // Rx DEM test Register 1
        debug!("Setting Rx DEM test Register 1");
        self.blocking_write_bytes(0x29, &[0x47]);

        // Rx DEM test Register 2
        debug!("Setting Rx DEM test Register 2");
        self.blocking_write_bytes(0x2a, &[0x80]);

        // Charge Pump
        debug!("Setting Charge Pump Current Register");
        self.blocking_write_bytes(0x2b, &[0x03]);

        // Crystal Test
        debug!("Setting Crystal Test Register");
        self.blocking_write_bytes(0x2c, &[0x01]);

        // PLL Test
        debug!("Setting PLL Test Register");
        self.blocking_write_bytes(0x2d, &[0x45]);

        // VCO Test 1
        debug!("Setting VCO Test Register 1");
        self.blocking_write_bytes(0x2e, &[0x18]);

        // VCO Test 2
        debug!("Setting VCO Test Register 2");
        self.blocking_write_bytes(0x2f, &[0x00]);

        // IFAT Register
        debug!("Setting IFAT Register");
        self.blocking_write_bytes(0x30, &[0x01]);

        // RScale Register
        debug!("Setting RScale Register");
        self.blocking_write_bytes(0x31, &[0x0f]);

        // Set to Standby mode
        self.blocking_strobe(Strobe::Standby);

        // Calibrate IF filter bank
        debug!("IF filter bank calibration...");
        self.blocking_write_bytes(0x02, &[0x01]);
        self.blocking_wait_auto_clear(0x02, 0x01);

        debug!("VCO Current Calibration");
        //Recomended calibration from A7105 Datasheet
        self.blocking_write_bytes(0x24, &[0x13]);

        debug!("VCO bank calibration");
        self.blocking_write_bytes(0x26, &[0x3b]);

        debug!("VCO Bank Calibrate channel 0");
        self.blocking_write_bytes(0x0f, &[0]); // set channel
        self.blocking_write_bytes(0x02, &[0x02]);
        self.blocking_wait_auto_clear(0x02, 0x02);

        let cal0 = &mut [0];
        self.blocking_read_bytes(0x25, cal0);
        if (cal0[0] & 0x08) != 0 {
            warn!("!!! VCO Calibration fail");
        }

        debug!("VCO Bank Calibrate channel 0xa0");
        self.blocking_write_bytes(0x0f, &[0xa0]); // set channel
        self.blocking_write_bytes(0x02, &[0x02]);
        self.blocking_wait_auto_clear(0x02, 0x02);

        let cal0 = &mut [0];
        self.blocking_read_bytes(0x25, cal0);
        debug!("vco cal a0={:02x}", cal0[0]);
        if (cal0[0] & 0x08) != 0 {
            debug!("!!! VCO Calibration fail");
        }

        debug!("Reset VCO band calibration");
        self.blocking_write_bytes(0x25, &[0x08]);

        debug!("Strobe standby");
        self.blocking_strobe(Strobe::Standby);
        self.register_dump();
        debug!("End of radio_init");
    }

    fn register_dump(&mut self) {
        let id = &mut [0, 0, 0, 0];
        self.blocking_read_bytes(0x06, id);
        debug!("Dumped ID: {:?}", id);
    }

    async fn strobe(&mut self, cmd: Strobe) {
        let byte = match cmd {
            Strobe::Sleep => 0b1000_0000,
            Strobe::Idle => 0b1001_0000,
            Strobe::Standby => 0b1010_0000,
            Strobe::Pll => 0b1011_0000,
            Strobe::Rx => 0b1100_0000,
            Strobe::Tx => 0b1101_0000,
            Strobe::FifoWritePointerReset => 0b1110_0000,
            Strobe::FifoReadPointerReset => 0b1111_0000,
        };

        self.write_bytes(byte, &[]).await;
    }

    fn blocking_strobe(&mut self, cmd: Strobe) {
        let byte = match cmd {
            Strobe::Sleep => 0b1000_0000,
            Strobe::Idle => 0b1001_0000,
            Strobe::Standby => 0b1010_0000,
            Strobe::Pll => 0b1011_0000,
            Strobe::Rx => 0b1100_0000,
            Strobe::Tx => 0b1101_0000,
            Strobe::FifoWritePointerReset => 0b1110_0000,
            Strobe::FifoReadPointerReset => 0b1111_0000,
        };

        self.blocking_write_bytes(byte, &[]);
    }

    async fn write_bytes(&mut self, mut addr: u8, bytes: &[u8]) {
        // Mask off the bits indicating it is a write
        addr &= 0x3f;

        self.cs.set_low();
        self.spi.write(&[addr]).await.ok();
        self.spi.write(bytes).await.ok();
        self.cs.set_high();
    }

    fn blocking_write_bytes(&mut self, mut addr: u8, bytes: &[u8]) {
        // Mask off the bits indicating it is a write
        addr &= 0x3f;

        self.cs.set_low();
        self.spi.blocking_write(&[addr]).ok();
        self.spi.blocking_write(bytes).ok();
        self.cs.set_high();
    }

    async fn read_bytes(&mut self, mut addr: u8, bytes: &mut [u8]) {
        // Mask off the bits indicating it is a read
        addr = (addr & 0x3f) | 0x40;

        self.cs.set_low();
        self.spi.write(&[addr]).await.ok();
        self.spi.read(bytes).await.ok();
        self.cs.set_high();
    }

    fn blocking_read_bytes(&mut self, mut addr: u8, bytes: &mut [u8]) {
        // Mask off the bits indicating it is a read
        addr = (addr & 0x3f) | 0x40;

        self.cs.set_low();
        self.spi.blocking_write(&[addr]).ok();
        self.spi.blocking_read(bytes).ok();
        self.cs.set_high();
    }

    async fn wait_auto_clear(&mut self, addr: u8, bit: u8) {
        let byte = &mut [0];
        loop {
            self.read_bytes(addr, byte).await;
            if (byte[0] & bit) == 0 {
                break;
            }
        }
    }

    fn blocking_wait_auto_clear(&mut self, addr: u8, bit: u8) {
        let byte = &mut [0];
        loop {
            self.blocking_read_bytes(addr, byte);
            if (byte[0] & bit) == 0 {
                break;
            }
        }
    }
}
