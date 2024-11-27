// use std::fs::File;
// use std::io::prelude::*;
// use rppal::i2c::I2c;

// use std::thread;
// use std::time::Duration;

use std::{fs::read, io::{self, Write}};
use schema::HIDBuffer;

mod device;
mod schema;
mod bt_mouse;
mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::init();

    let cfg = config::load()?;

    let mut device_hid = device::Device::new();
    
    println!("{:?}", cfg);

    let mut bt_mouse_input = bt_mouse::BtMouseInput::new(
        &cfg.bt_input_device.mouse_path
    )?;

    loop {
        bt_mouse_input.fetch();
        let hidbuf: HIDBuffer = bt_mouse_input.into_hid_buf();
        device_hid.send(&hidbuf);
    }
    
}
