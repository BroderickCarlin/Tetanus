#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use a7105::A7105;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::spi::{Config, Instance, RxDma, Spi, TxDma};
use embassy_stm32::time::Hertz;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _}; // global logger

mod a7105;

async fn spi_thing<P: Instance, Tx: TxDma<P>, Rx: RxDma<P>>(
    spi: &mut Spi<'_, P, Tx, Rx>,
    rx_buf: &mut [u8],
    tx_buf: &[u8],
) {
    spi.transfer(rx_buf, tx_buf).await.ok();
    info!("read via spi+dma: {}", &rx_buf);
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut a7105 = A7105::new(p.SPI1, p.PA5, p.PA7, p.PA6, p.PB6, p.DMA2_CH3, p.DMA2_CH2);
    a7105.init().await;
    info!("Initialized radio!");
    loop {
        info!("loop");
        // led.set_high();
        Timer::after(Duration::from_secs(2)).await;

        a7105.init().await;

        // info!("low");
        // // led.set_low();
        // Timer::after(Duration::from_millis(300)).await;
    }
}
