// use std::fs::File;
// use std::io::prelude::*;
// use rppal::i2c::I2c;

// use std::thread;
// use std::time::Duration;

use std::io::{self, Write};
use schema::HIDBuffer;

mod device;
mod schema;

fn main() {
    let mut device_hid = device::Device::new();

    // let kb = schema::KeyboardBuf {
    //     keys: [1, 1, 1, 1, 1, 1],
    //     ..Default::default()
    // };

    // let buf: HIDBuffer = kb.into();

    // device_hid.send(&buf);

    // let buf: HIDBuffer = schema::KeyboardBuf {
    //     modifier: 1,
    //     ..Default::default()
    // }
    // .into();

    // device_hid.send(&buf);

    // let buf: HIDBuffer = schema::RelaMouseBuf {
    //     x_movement: 100,
    //     y_movement: -100,
    //     ..Default::default()
    // }
    // .into();

    // device_hid.send(&buf);

    loop {
        print!("x y >>");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let loc: Vec<u16> = input.split(" ")
        .map(|v: &str| v.parse::<u16>().unwrap()).collect();

        let buf: HIDBuffer = schema::AbslMouseBuf {
            x_position: loc[0],
            y_position: loc[1],
            ..Default::default()
        }
        .into();
    
        device_hid.send(&buf);
    }

}
