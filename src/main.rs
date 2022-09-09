extern crate anyhow;
extern crate esp_idf_hal;
extern crate esp_idf_sys;

use embedded_hal::digital::v2::{InputPin, OutputPin};
use esp_idf_hal::gpio::Pull;
use esp_idf_hal::peripherals::Peripherals;

use std::thread::sleep;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let pin25 = pins.gpio25.into_output()?;
    let pin32 = pins.gpio32.into_output()?;
    let pin27 = pins.gpio27.into_output()?;

    let mut cols = [
        pin25.degrade(),
        pin32.degrade(),
        pin27.degrade(),
    ];

    let mut pin33 = pins.gpio33.into_input()?;
    let mut pin12 = pins.gpio12.into_input()?;
    let mut pin14 = pins.gpio14.into_input()?;
    let mut pin26 = pins.gpio26.into_input()?;

    pin33.set_pull_up()?;
    pin12.set_pull_up()?;
    pin14.set_pull_up()?;
    pin26.set_pull_up()?;

    let mut rows = [
        pin33.degrade(),
        pin12.degrade(),
        pin14.degrade(),
        pin26.degrade(),
    ];

    let charmap = [
        ['1', '2', '3'],
        ['4', '5', '6'],
        ['7', '8', '9'],
        ['*', '0', '#'],
    ];

    loop {
        for (col, col_pin) in cols.iter_mut().enumerate() {
            col_pin.set_low()?;

            for (row, row_pin) in rows.iter_mut().enumerate() {
                let pressed = row_pin.is_low()?;
                if pressed {
                    let ch = charmap[row][col];
                    println!("{ch}");

                    sleep(Duration::from_millis(200));
                }
            }

            col_pin.set_high()?;
        }
    }
}
