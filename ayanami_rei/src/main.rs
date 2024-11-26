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

    // loop {
    //     print!("x y >>");
    //     io::stdout().flush().unwrap();
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).unwrap();
    //     let input = input.trim();

    //     let loc: Vec<i16> = input.split(" ")
    //     .map(|v: &str| v.parse::<i16>().unwrap()).collect();

    //     let buf: HIDBuffer = schema::AbslMouseBuf {
    //         x_position: loc[0],
    //         y_position: loc[1],
    //         ..Default::default()
    //     }
    //     .into();
    
    //     device_hid.send(&buf);
    // }

    
    loop {
        for i in 0..10 {
            for j in 0..10 {
                let x: i16 = (j as f32 * (3276.7 as f32)) as i16;
                let y: i16 = (i as f32 * (3276.7 as f32)) as i16;

                let buf: HIDBuffer = schema::AbslMouseBuf {
                    x_position: x,
                    y_position: y,
                    ..Default::default()
                }
                .into();
            
                device_hid.send(&buf);

                std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
            }
        }
    }


}
