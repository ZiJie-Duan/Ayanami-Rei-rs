use std::default;
use std::os::raw::{c_long, c_ulong};
use std::fs::File;
use std::io::{self, Read};
use std::mem::size_of;
use log::{error, debug};

use crate::schema::{HIDBuffer, RelaMouseBuf};

#[repr(C)]
#[derive(Debug)]
struct TimeVal {
    tv_sec: c_long,    // seconds
    tv_usec: c_long,   // microseconds
}

#[repr(C)]
#[derive(Debug)]
struct InputEvent {
    time: TimeVal,     // timestamp
    type_: u16,        // event type
    code: u16,         // event code
    value: i32,        // event value
}

enum MouseInput {
    MoveX(i32),
    MoveY(i32),
    LeftButtonClick(bool),
    RightButtonClick(bool),
    CustomButton1(bool),
    CustomButton2(bool),
    None,
}

pub struct BtMouseInput{
    mouse_input: MouseInput,
    file: File,
    buffer: [u8; std::mem::size_of::<InputEvent>()],
}

impl BtMouseInput {

    pub fn into_hid_buf(&self) -> HIDBuffer{
        match self.mouse_input {
            MouseInput::MoveX(v) => 
                HIDBuffer::RelaMouse(
                    RelaMouseBuf{
                        x_movement:(v.clamp(i8::MIN as i32, i8::MAX as i32) as f32 * 1.1) as i8, 
                        ..Default::default()}
                ),
            MouseInput::MoveY(v) => 
                HIDBuffer::RelaMouse(
                    RelaMouseBuf{
                        y_movement:(v.clamp(i8::MIN as i32, i8::MAX as i32) as f32 * 0.65) as i8,
                         ..Default::default()}
                ),
            MouseInput::LeftButtonClick(v) => {
                    if v {
                        HIDBuffer::RelaMouse(
                            RelaMouseBuf{button_status:1, ..Default::default()}
                        )
                    } else {
                        HIDBuffer::RelaMouse(
                            RelaMouseBuf{button_status:0, ..Default::default()}
                        )
                    }
                },
            MouseInput::RightButtonClick(v) => {
                    if v {
                        HIDBuffer::RelaMouse(
                            RelaMouseBuf{button_status:2, ..Default::default()}
                        )
                    } else {
                        HIDBuffer::RelaMouse(
                            RelaMouseBuf{button_status:0, ..Default::default()}
                        )
                    }
                },
            _ => HIDBuffer::RelaMouse(
                RelaMouseBuf::default()
            ),
        }
    }
        
    pub fn new(path:&str) -> Result<Self, io::Error> {
        Ok(Self {
            mouse_input: MouseInput::None,
            file: match File::open(path) {
                Ok(file) => file,
                Err(e) => {
                    error!("Failed to open Bluetooth mouse device file: {}", e);
                    return Err(e);
                },
            },
            buffer: [0u8; std::mem::size_of::<InputEvent>()],
        })
    }

    fn decode(&mut self, event: InputEvent){
        match event.type_ {
            1 => match event.code {
                0x110 => match event.value {
                    0 => self.mouse_input = MouseInput::LeftButtonClick(false),
                    1 => self.mouse_input = MouseInput::LeftButtonClick(true),
                    _ => error!("unexpected left mouse button state")
                },
                0x111 => match event.value {
                    0 => self.mouse_input = MouseInput::RightButtonClick(false),
                    1 => self.mouse_input = MouseInput::RightButtonClick(true),
                    _ => error!("unexpected left mouse button state")
                }
                _ => {},
            }
            2 => match event.code {
                0x00 => {
                    self.mouse_input = MouseInput::MoveX(event.value);
                },
                0x01 => {
                    self.mouse_input = MouseInput::MoveY(event.value);
                },
                _ => {},
            },
            _ => {},
        }
    }

    pub fn fetch(&mut self) -> Result<(), io::Error>{
        match self.file.read_exact(&mut self.buffer) {
            Ok(_) => {

                let event = unsafe { std::ptr::read(self.buffer.as_ptr() as *const InputEvent) };
                debug!(
                    "Time: {}.{}\tType: {}\tCode: {}\tValue: {}",
                    event.time.tv_sec, event.time.tv_usec, event.type_, event.code, event.value
                );

                // decode event here
                self.decode(event);

                Ok(())
            },
            Err(e) => {
                error!("Error reading event: {}", e);
                Err(e)
            }
        }
    }
}
