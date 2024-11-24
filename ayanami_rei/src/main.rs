// use std::fs::File;
// use std::io::prelude::*;
// use rppal::i2c::I2c;

// use std::thread;
// use std::time::Duration;

use schema::HIDBuffer;

mod device;
mod schema;

fn main() {
    let mut device_hid = device::Device::new();

    let kb = schema::KeyboardBuf {
        keys: [1, 1, 1, 1, 1, 1],
        ..Default::default()
    };

    let buf: HIDBuffer = kb.into();

    device_hid.send(&buf);

    let buf: HIDBuffer = schema::KeyboardBuf {
        modifier: 1,
        ..Default::default()
    }
    .into();

    device_hid.send(&buf);

    let buf: HIDBuffer = schema::RelaMouseBuf {
        x_movement: 10,
        y_movement: -10,
        ..Default::default()
    }
    .into();

    device_hid.send(&buf);

    let buf: HIDBuffer = schema::AbslMouseBuf {
        x_position: 1920,
        y_position: 1080,
        ..Default::default()
    }
    .into();

    device_hid.send(&buf);
}
