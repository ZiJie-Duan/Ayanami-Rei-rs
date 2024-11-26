use std::default;
use std::os::raw::{c_long, c_ulong};
use std::fs::File;
use std::io::{self, Read};
use std::mem::size_of;
use log::{error, debug};

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
        
    pub fn new(path:&str) -> Result<Self, ()> {
        Ok(Self {
            mouse_input: MouseInput::None,
            file: match File::open(path) {
                Ok(file) => file,
                Err(e) => {
                    error!("Failed to open Bluetooth mouse device file: {}", e);
                    return Err(());
                },
            },
            buffer: [0u8; std::mem::size_of::<InputEvent>()],
        })
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

                Ok(())
            },
            Err(e) => {
                error!("Error reading event: {}", e);
                Err(e)
            }
        }
    }
}
