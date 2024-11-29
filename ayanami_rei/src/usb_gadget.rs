use log::debug;
use std::{fs::File, io::Write};

use crate::config::Config;
use crate::mouse::{MouseOut};

pub trait DeviceBuffer {
    fn to_vec(&self) -> Vec<u8>;
}

#[derive(Default)]
pub struct KeyboardBuf {
    pub modifier: u8,  // Modifier key
    pub reserved: u8,  // Reserved byte
    pub keys: [u8; 6], // Key values of regular keys pressed
}

impl DeviceBuffer for KeyboardBuf {
    fn to_vec(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.push(self.modifier);
        v.push(self.reserved);
        self.keys.iter().for_each(|e| v.push(*e));
        v
    }
}

#[derive(Default)]
pub struct RelaMouseBuf {
    pub button_status: u8, // Button status
    pub x_movement: i8,    // Change in X coordinate
    pub y_movement: i8,    // Change in Y coordinate
    pub v_wheel: i8,       // Change in vertical wheel
    pub h_wheel: i8,       // Change in horizontal wheel
}

impl DeviceBuffer for RelaMouseBuf {
    fn to_vec(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.push(self.button_status);
        v.push(self.x_movement as u8);
        v.push(self.y_movement as u8);
        v.push(self.v_wheel as u8);
        v.push(self.h_wheel as u8);
        v
    }
}

#[derive(Default)]
pub struct AbslMouseBuf {
    pub button_status: u8, // Button status
    pub x_position: i16,   // Absolute position of X coordinate
    pub y_position: i16,   // Absolute position of Y coordinate
    pub v_wheel: i8,       // Change in vertical wheel
    pub h_wheel: i8,       // Change in horizontal wheel
}

impl DeviceBuffer for AbslMouseBuf {
    fn to_vec(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.push(self.button_status);
        v.extend(self.x_position.to_le_bytes().iter());
        v.extend(self.y_position.to_le_bytes().iter());
        v.push(self.v_wheel as u8);
        v.push(self.h_wheel as u8);
        v
    }
}

pub enum HIDBuffer {
    Keyboard(KeyboardBuf),
    RelaMouse(RelaMouseBuf),
    AbslMouse(AbslMouseBuf),
}

impl From<KeyboardBuf> for HIDBuffer {
    fn from(value: KeyboardBuf) -> Self {
        Self::Keyboard(value)
    }
}

impl From<AbslMouseBuf> for HIDBuffer {
    fn from(value: AbslMouseBuf) -> Self {
        Self::AbslMouse(value)
    }
}

impl From<RelaMouseBuf> for HIDBuffer {
    fn from(value: RelaMouseBuf) -> Self {
        Self::RelaMouse(value)
    }
}

impl DeviceBuffer for HIDBuffer {
    fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::RelaMouse(buf) => buf.to_vec(),
            Self::AbslMouse(buf) => buf.to_vec(),
            Self::Keyboard(buf) => buf.to_vec(),
        }
    }
}

// const KEYBOARD_PATH: &str = "/dev/hidg0";
// const RELA_MOUSE_PATH: &str = "/dev/hidg1";
// const ABSL_MOUSE_PATH: &str = "/dev/hidg2";

pub struct Device {
   keyboard: File,
   rela_mouse: File,
   abs_mouse: File,

   x_speed: f32,  // HID Mouse X Speed, default is 1
   y_speed: f32,  // HID Mouse Y Speed, default is 1
   x_screen_range: i32, // Maximum number of pixels on the X screen
   y_screen_range: i32, // Maximum number of pixels on the Y screen
   x_hid_range: i32, // Valid report range for HID
   y_hid_range: i32, // Valid report range for HID
   x_map_rate: f32, // Mapping ratio of pixels to HID mouse values on the X axis
   y_map_rate: f32, // Mapping ratio of pixels to HID mouse values on the Y axis
}

impl Device {
    pub fn new(cfg: &Config) -> Self {
        Self {
            keyboard: File::create(cfg.hid_device.keyboard_path.clone()).unwrap(),
            rela_mouse: File::create(cfg.hid_device.rela_mouse_path.clone()).unwrap(),
            abs_mouse: File::create(cfg.hid_device.abs_mouse_path.clone()).unwrap(),
            x_speed: cfg.hid_mouse_setting.x_speed,
            y_speed: cfg.hid_mouse_setting.y_speed,
            x_screen_range: cfg.hid_mouse_setting.x_screen_range,
            y_screen_range: cfg.hid_mouse_setting.y_screen_range,
            x_hid_range: cfg.hid_mouse_setting.x_hid_range,
            y_hid_range: cfg.hid_mouse_setting.y_hid_range,
            x_map_rate: cfg.hid_mouse_setting.x_screen_range as f32 / cfg.hid_mouse_setting.x_hid_range as f32,
            y_map_rate: cfg.hid_mouse_setting.y_screen_range as f32 / cfg.hid_mouse_setting.y_hid_range as f32,
        }
    }

    /// Helper function to convert a vector of bytes to a binary string.
    fn to_binary_string(vec: Vec<u8>) -> String {
        vec.iter()
            .map(|byte| format!("{:08b}", byte))
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn send(&mut self, buf: &HIDBuffer) {
        match buf {
            HIDBuffer::Keyboard(buf) => {
                debug!(
                    "Send to Keyboard {:?}",
                    Self::to_binary_string(buf.to_vec())
                );
            }
            HIDBuffer::RelaMouse(buf) => {
                debug!(
                    "Send to RelaMouse {:?}",
                    Self::to_binary_string(buf.to_vec())
                );
            }
            HIDBuffer::AbslMouse(buf) => {
                debug!(
                    "Send to AbslMouse {:?}",
                    Self::to_binary_string(buf.to_vec())
                );
            }
        }
        match buf {
           HIDBuffer::Keyboard(buf) => self.keyboard.write_all(&(buf.to_vec())).unwrap(),
           HIDBuffer::RelaMouse(buf) => self.rela_mouse.write_all(&(buf.to_vec())).unwrap(),
           HIDBuffer::AbslMouse(buf) => self.abs_mouse.write_all(&(buf.to_vec())).unwrap(),
        }
    }
    
    fn map_loc(&self, x: i32, y: i32) -> (i32, i32) {
        ((x as f32 * self.x_map_rate) as i32, (y as f32 * self.y_map_rate) as i32)
    }

    fn move_loc(&self, x_det:i32, y_det:i32) -> (i32, i32) {
        ((x_det as f32 *self.x_speed) as i32, (y_det as f32 *self.y_speed) as i32)
    }

}



impl MouseOut for Device {
    fn mouse_move(&mut self, x:i32, y:i32) {
        self.send(&HIDBuffer::RelaMouse(RelaMouseBuf{
            x_movement: x.clamp(i8::MIN as i32, i8::MAX as i32) as i8,
            y_movement: y.clamp(i8::MIN as i32, i8::MAX as i32) as i8,
            ..Default::default()
        }));
    }
}