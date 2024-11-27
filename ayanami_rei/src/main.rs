// use std::fs::File;
// use std::io::prelude::*;
// use rppal::i2c::I2c;

// use std::thread;
// use std::time::Duration;

use std::{fs::read, io::{self, Write}};
use usb_gadget::HIDBuffer;

mod device;
mod usb_gadget;
mod bt_mouse;
mod config;
mod mouse;
mod memo;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::init();

    let cfg = config::load()?;

    let mut mousein = bt_mouse::BtMouseInput::new(
            &cfg.bt_input_device.mouse_path
        )?;
    
    let mut mouseout = usb_gadget::Device::new(&cfg);

    let mut mouse = mouse::Mouse::new(&cfg, Box::new(mousein), Box::new(mouseout));

    loop{
        mouse.update();
    }
    
    // println!("{:?}", cfg);

    // let mut bt_mouse_input = bt_mouse::BtMouseInput::new(
    //     &cfg.bt_input_device.mouse_path
    // )?;

    // loop {
    //     let now = std::time::Instant::now();
    //     bt_mouse_input.fetch();
    //     let hidbuf: HIDBuffer = bt_mouse_input.into_hid_buf();
    //     device_hid.send(&hidbuf);
    //     println!("Took {}us", now. elapsed().as_micros());
    // }
    
}
