use defmt::{debug, unwrap};
use embassy_stm32::{
    gpio::{Level, Output, Pin, Speed},
    spi::{Config, Instance, MisoPin, MosiPin, RxDma, SckPin, Spi, TxDma},
    time::Hertz,
    Peripheral,
};
use embassy_time::{Duration, Timer};

pub mod commands;

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
        debug!("Resetting the a7105");
        // Wait for it to be ready, in case we just powered on.
        Timer::after(Duration::from_millis(25)).await;

        // Zero - reset register.
        self.write_bytes(0x0, &[0x00]).await;
        Timer::after(Duration::from_millis(50)).await;

        // Configure GPIO1 as our MISO pin
        debug!("Configuring GPIO 1 Pin...");
        self.write_bytes(0xB, &[0b00_0110_01]).await;

        let mut errs: usize = 0;
        let mut cnt: usize = 0;
        loop {
            // Write the ID
            self.write_bytes(0x6, &[0x54, 0x75, 0xc5, 0x2a]).await;

            let read_buf = &mut [0, 0, 0, 0];
            self.read_bytes(0x6, read_buf).await;

            cnt += 1;
            if read_buf != &[0x54, 0x75, 0xc5, 0x2a] {
                errs += 1;
                debug!(
                    "Found an ID mismatch! {}/{} {:?} vs {:?}",
                    errs,
                    cnt,
                    read_buf,
                    &[0x54, 0x75, 0xc5, 0x2a]
                );
            }

            if cnt % 1000 == 0 {
                debug!("iter {}", cnt);
            }
        }

        return;

        // debug!("Writing config data");
        // let reg_init_values = &[
        //     // NOTE: Registers 0xb and 0xc control GIO1 and GIO2, set them to 0 for now.
        //     0xFF, 0x42, 0x00, 0x25, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05,
        //     0x00, 0x50, // 00 - 0f
        //     0x9e, 0x4b, 0x00, 0x02, 0x16, 0x2b, 0x12, 0x00, 0x62, 0x80, 0xFF, 0xFF, 0x2a, 0x32,
        //     0xc3, 0x1f, // 10 - 1f
        //     0x13, 0xc3, 0x00, 0xFF, 0x00, 0x00, 0x3b, 0x00, 0x17, 0x47, 0x80, 0x03, 0x01, 0x45,
        //     0x18, 0x00, // 20 - 2f
        //     0x01, 0x0fu8, // 30 - 31
        // ];

        // for (addr, val) in reg_init_values
        //     .iter()
        //     .enumerate()
        //     .filter(|(_, val)| **val != 0xFF)
        // {
        //     debug!("write byte: 0x{:02X}: 0x{:02X}", addr, val);
        //     self.write_byte(addr as u8, *val);
        // }

        // // Configure GPIO1 as our MISO pin
        // debug!("Configuring GPIO 1 Pin...");
        // self.write_byte(0xB, 0b00_0110_01);
        // Timer::after(Duration::from_millis(50)).await;

        // for n in 0..0x30u8 {
        //     let b = self.read_byte(n);
        //     if b != reg_init_values[n as usize] && reg_init_values[n as usize] != 0xFF {
        //         debug!(
        //             "read byte: 0x{:02X}: 0x{:02X} vs 0x{:02X}",
        //             n, b, reg_init_values[n as usize]
        //         );
        //     }
        // }
    }

    async fn write_bytes(&mut self, mut addr: u8, bytes: &[u8]) {
        // Mask off the bits indicating it is a write
        addr &= 0x3f;

        self.cs.set_low();
        self.spi.write(&[addr]).await.ok();
        self.spi.write(bytes).await.ok();
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

    // async fn write_byte(&mut self, addr: u8, byte: u8) {
    //     let addr = addr & 0x3f;
    //     self.cs.set_low();
    //     unwrap!(self.spi.blocking_transfer_in_place(&mut [addr, byte]));
    //     self.cs.set_high();
    // }

    // fn read_byte(&mut self, addr: u8) -> u8 {
    //     let data = &mut [0];
    //     self.read_block(addr, data);
    //     data[0]
    // }

    // fn write_block(&mut self, mut addr: u8, bytes: &mut [u8]) {
    //     addr &= 0x3f;
    //     self.cs.set_low();
    //     unwrap!(self.spi.blocking_transfer_in_place(&mut [addr]));
    //     unwrap!(self.spi.blocking_transfer_in_place(bytes));
    //     self.cs.set_high();
    // }

    // fn read_block(&mut self, addr: u8, bytes: &mut [u8]) {
    //     let addr = (addr & 0x3f) | 0x40; // Set read flag.

    //     self.cs.set_low();
    //     unwrap!(self.spi.blocking_transfer_in_place(&mut [addr]));
    //     unwrap!(self.spi.blocking_transfer_in_place(bytes));
    //     self.cs.set_high();
    // }
}

// //A7105 registers values -> ROM
// // magic value 0xff means don't write.
// static const unsigned char reg_init_values[] = {
//     // NOTE: Registers 0xb and 0xc control GIO1 and GIO2, set them to 0 for now.
//     0xFF, 0x42 , 0x00, 0x25, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x50,        // 00 - 0f
//     0x9e, 0x4b, 0x00, 0x02, 0x16, 0x2b, 0x12, 0x00, 0x62, 0x80, 0xFF, 0xFF, 0x2a, 0x32, 0xc3, 0x1f,                         // 10 - 1f
//     0x13, 0xc3, 0x00, 0xFF, 0x00, 0x00, 0x3b, 0x00, 0x17, 0x47, 0x80, 0x03, 0x01, 0x45, 0x18, 0x00,                         // 20 - 2f
//     0x01, 0x0f // 30 - 31
// };

// static void program_config()
// {
//     for (uint8_t addr =0; addr < sizeof(reg_init_values); addr ++) {
//         uint8_t v = reg_init_values[addr];
//         if (v != 0xff) {
//             spi_write_byte(addr, v);
//         }
//     }
// }
