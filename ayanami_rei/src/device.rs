use crate::schema::{DeviceBuffer, HIDBuffer};
use std::{fs::File, io::Write};

const KEYBOARD_PATH: &str = "/dev/hidg0";
const RELA_MOUSE_PATH: &str = "/dev/hidg1";
const ABSL_MOUSE_PATH: &str = "/dev/hidg2";

//pub struct Device {
//    keyboard: File,
//    rela_mouse: File,
//    abs_mouse: File,
//}

//impl Default for Device {
//    fn default() -> Self {
//        Self {
//            keyboard: File::create(KEYBOARD_PATH).unwrap(),
//            rela_mouse: File::create(RELA_MOUSE_PATH).unwrap(),
//            abs_mouse: File::create(ABSL_MOUSE_PATH).unwrap(),
//        }
//    }
//}
//

pub struct Device {
    keyboard: String,
    rela_mouse: String,
    abs_mouse: String,
}

impl Default for Device {
    fn default() -> Self {
        Self {
            keyboard: "Keyboard".to_string(),
            rela_mouse: "RelaMouse".to_string(),
            abs_mouse: "AbsMouse".to_string(),
        }
    }
}

impl Device {
    pub fn new() -> Self {
        Self::default()
    }

    /// Helper function to convert a vector of bytes to a binary string.
    fn to_binary_string(vec: Vec<u8>) -> String {
        vec.iter()
            .map(|byte| format!("{:08b}", byte))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn send(&mut self, buf: &HIDBuffer) {
        match &buf {
            HIDBuffer::Keyboard(buf) => {
                println!(
                    "Send to Keyboard {:?}",
                    Self::to_binary_string(buf.to_vec())
                );
            }
            HIDBuffer::RelaMouse(buf) => {
                println!(
                    "Send to RelaMouse {:?}",
                    Self::to_binary_string(buf.to_vec())
                );
            }
            HIDBuffer::AbslMouse(buf) => {
                println!(
                    "Send to AbslMouse {:?}",
                    Self::to_binary_string(buf.to_vec())
                );
            }
        }
        //match buf {
        //    HIDBuffer::Keyboard(buf) => self.keyboard.write_all(&(buf.to_vec())).unwrap(),
        //    HIDBuffer::RelaMouse(buf) => self.rela_mouse.write_all(&(buf.to_vec())).unwrap(),
        //    HIDBuffer::AbslMouse(buf) => self.abs_mouse.write_all(&(buf.to_vec())).unwrap(),
        //}
    }
}
