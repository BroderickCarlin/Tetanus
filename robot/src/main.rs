#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

// use a7105::commands::Strobe;
// use a7105::A7105;
// use defmt::*;
use embassy_executor::Spawner;
// use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _}; // global logger

// mod radio;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    // info!("Hello World!");

    // let mut a7105 = A7105::new(
    //     p.SPI1, p.PA5, p.PA7, p.PA6, p.PB6, p.DMA2_CH3, p.DMA2_CH2, p.PA9,
    // );
    // a7105.init().await;

    // debug!("Scanning channels...");
    // // let mut chan = 0x8bu8;
    // // let mut counter = 0u32;
    // // let mut scanning = true;
    // // a7105.blocking_write_then_strobe(Strobe::Rx, 0x0f, &[chan - 1]);
    // // loop {
    // //     let got_packet = a7105.wait_gpio().await;
    // //     if got_packet {
    // //         let modeflags = &mut [0];
    // //         a7105.blocking_read_register(0x00, modeflags);

    // //         if (modeflags[0] & (1 << 6) | (1 << 5)) == 0 {
    // //             // No errors, so read away!
    // //             let buf = &mut [0; 37];
    // //             a7105.blocking_strobe_then_read(Strobe::FifoReadPointerReset, 0x05, buf);
    // //             let packet_type = buf[0];

    // //             if packet_type == 0x58 {
    // //                 // sticks; ignore??
    // //                 debug!("found a sticks packet!");
    // //             } else if buf[5] == 0xff {
    // //                 // generic bind packet? ignore??
    // //                 debug!("found a bind packet!");
    // //             }
    // //         }

    // //         scanning = false;
    // //         counter += 1;
    // //     }

    // //     if scanning && ((counter >= 3) || (!got_packet)) {
    // //         /* Sequential  */
    // //         chan += 1;
    // //         if chan > 0xa0 {
    // //             chan = 1;
    // //         }
    // //         /* Bind channels are 0xd and 0x8c */
    // //         if chan != 0xc {
    // //             chan = 0xc;
    // //         } else {
    // //             chan = 0x8b;
    // //         }
    // //         counter = 0;
    // //     }

    // //     a7105.blocking_strobe(Strobe::Idle);
    // //     a7105.set_channel(chan);
    // //     a7105.blocking_strobe(Strobe::Rx);
    // //     // a7105.blocking_write_then_strobe(Strobe::Rx, 0x0f, &[chan - 1]);
    // // }

    // loop {
    //     Timer::after(Duration::from_millis(100)).await;
    //     for channel in &[0x0c, 0x8b] {
    //         // for ref channel in 0..0xa1 {
    //         // self.blocking_strobe(Strobe::Standby);
    //         a7105.set_channel(*channel);
    //         a7105.blocking_strobe(Strobe::Rx);

    //         for _ in 0..15 {
    //             if a7105.wait_gpio().await {
    //                 let mode_flags = &mut [0u8];
    //                 // self.spi.blocking_read(mode_flags).ok();
    //                 // self.blocking_read_bytes(mode_flags);
    //                 // a7105.blocking_strobe(Strobe::FifoReadPointerReset);
    //                 // a7105.blocking_read_register(0x0f, mode_flags);

    //                 a7105.blocking_read_register(0x00, mode_flags);
    //                 // I think flag 0x01 (bit 0) is the "read data ready" flag, active low,doc is not clear about this.
    //                 // bits 5 and 6 are read error flags.

    //                 // diag_println("modeflags=%02x endptr=%02x", (int) modeflags, (int) endptr);
    //                 let errflags: u8 = (1 << 6) | (1 << 5);

    //                 if mode_flags[0] & errflags != 0 {
    //                     debug!("got mode: 0b{:08b}", mode_flags[0]);
    //                 } else {
    //                     // Read buffer
    //                     let buf = &mut [0u8; 37];
    //                     a7105.blocking_strobe(Strobe::FifoReadPointerReset);
    //                     a7105.blocking_read_register(0x05, buf);

    //                     // if buf != &[0u8; 37] {
    //                     //     debug!("c={:02x} modeflags={:02x} data=", channel, mode_flags);
    //                     //     debug!("packet: {:?}", buf);
    //                     // }

    //                     // if buf[0] == 0x58 {
    //                     //     debug!("chan={:02x} sticks packet", channel);
    //                     //     break;
    //                     // } else if buf[0] == 0xbb || buf[0] == 0xbc {
    //                     //     debug!("chan={:02x} bind packet", channel);
    //                     // } else {
    //                     //     debug!("chan={:02x} unknown packet 0x{:02X} {}", channel, buf[0], buf);
    //                     // }

    //                     if let Ok((_, packet)) = radio::TransmitterPacket::from_bytes(buf) {
    //                         debug!("got packet: \n\n {:#?}", packet);
    //                     } else {
    //                         debug!("got invalid packet: {}", buf);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}
