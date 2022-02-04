use crate::utils;

//  ======
//  ENTITY
//  ======

#[derive(Debug)]
pub struct Entity {
    x: i32,
    y: i32,
    speed: i32,
    color: utils::RGBColor,
    symbol: String,
    frame_count: i32,
    switch_interval: i32,
}

impl Entity {
    /// Entity constructor
    pub fn new(x: i32, y: i32, speed: i32, color: utils::RGBColor, is_first: bool) -> Self {
        Self {
            x,
            y,
            speed,
            color: if is_first {
                utils::RGBColor(200, 255, 200)
            } else {
                color
            },
            symbol: String::from("x"),
            frame_count: 0,
            switch_interval: 20,
        }
    }

    /// Set Entity Symbol
    pub fn set_symbol(&mut self) {
        let r = utils::random_between(0, 1);
        self.symbol = if r == 0 {
            String::from("0")
        } else {
            String::from("1")
        };
    }

    /// Rain
    pub fn rain(&mut self, rows: i32, speed_offset: i32) {
        self.y = if self.y > rows {
            0
        } else {
            self.y + (self.speed + speed_offset) / 6
        }
    }

    /// Render
    pub fn render(&mut self) {
        utils::cursor_move_to(self.y, self.x);
        print!("{}", utils::rgb(&self.symbol, self.color));
        if self.frame_count % self.switch_interval == 0 {
            self.set_symbol()
        }
        self.frame_count += 1;
    }
}
