use log::{debug, error};

use crate::config::Config;

pub enum MouseMoveBuf {
    RelaMove(i32, i32),
    AbslMove(i32, i32),
}

pub trait MouseIn {
    fn mouse_move(&mut self) -> MouseMoveBuf;
}


pub trait MouseOut {
    fn mouse_move(&mut self, mouse_move_buf: MouseMoveBuf);
}


pub struct Mouse {
    x_position: i32,   // Mouse X Position
    y_position: i32,   // Mouse Y Position
    x_speed: f32,      // Mouse X Speed
    y_speed: f32,      // Mouse Y Speed
    x_range: i32,      // Mouse Movable Range, Equal to Screen Pixel Count
    y_range: i32,      // Mouse Movable Range, Equal to Screen Pixel Count

    obj_in: Box<dyn MouseIn>,
    obj_out: Box<dyn MouseOut>, 
}

impl Mouse {
    pub fn new(cfg: &Config, obj_in: Box<dyn MouseIn>, obj_out: Box<dyn MouseOut>) -> Self{
        Self {
            x_position: 0,
            y_position: 0,
            x_speed: cfg.mouse_setting.x_speed,
            y_speed: cfg.mouse_setting.y_speed,
            x_range: cfg.mouse_setting.x_range,
            y_range: cfg.mouse_setting.y_range,
            obj_in: obj_in,
            obj_out: obj_out,
        }
    }

    pub fn move_loc(&mut self, x:i32, y:i32){
        let x = x as f32;
        let y = y as f32;
        self.x_position = (self.x_position + x*self.x_speed).clamp(0.0, self.x_range as f32) as i32;
        self.y_position = (self.y_position + x*self.y_speed).clamp(0.0, self.y_range as f32) as i32;
    }


    pub fn set_loc(&mut self, x:i32, y:i32) {
        if x >= 0 && x <= self.x_range && y >= 0 && y <= self.y_range {
            self.x_position = x;
            self.y_position = y;
        } else {
            error!("Coordinates out of range: x = {}, y = {}", x, y);
        }
    }

    pub fn update(&mut self){
        let (x, y) = self.obj_in.mouse_move();
        //self.move_loc(x, y);
        debug!("Mouse Position x:{}, y:{}", self.x_position, self.y_position);
        self.obj_out.mouse_move(x, y);

    }
}


