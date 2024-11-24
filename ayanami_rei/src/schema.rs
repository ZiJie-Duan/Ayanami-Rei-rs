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
