#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut led_green = Output::new(p.PJ2, Level::High, Speed::Low);
    let mut led_red = Output::new(p.PI13, Level::High, Speed::Low);
    let mut led_lcd = Output::new(p.PK0, Level::High, Speed::Low);

    loop {
        info!("high");
        led_green.set_high();
        led_red.set_high();
        led_lcd.set_high();
        Timer::after(Duration::from_millis(100)).await;

        info!("low");
        led_green.set_low();
        led_red.set_low();
        led_lcd.set_low();
        Timer::after(Duration::from_millis(100)).await;
    }
}
