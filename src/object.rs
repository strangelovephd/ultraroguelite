use tcod::Color;
use tcod::Console;
use tcod::BackgroundFlag;
use std::any::Any;
// pub struct Objects<T> 
// where T: GameObject {
//     pub objs: Vec<T>,
// }

pub struct Objects {
    pub objs: Vec<Box<GameObject>>,
}

pub trait GameObject {
    fn move_by(&mut self, dx: i32, dy: i32);
    fn draw(&self, con: &mut Console);
    fn clear(&self, con: &mut Console);
}

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub ch: char,
    pub color: Color,
}

impl Player {
    pub fn new(x: i32, y: i32, ch: char, color: Color) -> Self {
        Self {
            x: x,
            y: y,
            ch: ch,
            color: color,
        }
    }
}

pub struct NPC {
    pub x: i32,
    pub y: i32,
    pub ch: char,
    pub color: Color,
}

impl NPC {
    pub fn new(x: i32, y: i32, ch: char, color: Color) -> Self {
        Self {
            x: x,
            y: y,
            ch: ch,
            color: color,
        }
    }
}

impl GameObject for Player {
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn draw(&self, con: &mut Console) {
        con.set_default_background(self.color);
        con.put_char(self.x, self.y, self.ch, BackgroundFlag::None);
    }

    fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}

impl GameObject for NPC {
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn draw(&self, con: &mut Console) {
        con.set_default_background(self.color);
        con.put_char(self.x, self.y, self.ch, BackgroundFlag::None);
    }

    fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}
