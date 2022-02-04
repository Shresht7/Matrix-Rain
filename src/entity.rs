use crate::utils;

//  ======
//  ENTITY
//  ======

pub struct Entity {
    x: i32,
    y: i32,
    speed: i32,
    color: utils::RGBColor,
    symbol: char,
    mode: utils::Mode,
    frame_count: i32,
    switch_interval: i32,
}

impl Entity {
    /// Entity constructor
    pub fn new(
        x: i32,
        y: i32,
        speed: i32,
        color: utils::RGBColor,
        mode: utils::Mode,
        is_first: bool,
    ) -> Self {
        Self {
            x,
            y,
            speed,
            color: if is_first {
                utils::RGBColor(200, 255, 200)
            } else {
                color
            },
            symbol: 'x',
            mode,
            frame_count: 0,
            switch_interval: 20,
        }
    }

    /// Set Entity Symbol
    pub fn set_symbol(&mut self) {
        match self.mode {
            utils::Mode::Original => {
                let r = utils::random_between(0x30a0, 0x30a0 + 96) as u32;
                self.symbol = std::char::from_u32(r).unwrap_or('0');
            }

            utils::Mode::Binary => {
                let r = utils::random_between(0, 2);
                self.symbol = if r == 0 { '0' } else { '1' };
            }

            utils::Mode::ASCII => {
                let r = utils::random_between(0, 127) as u32;
                self.symbol = std::char::from_u32(r).unwrap_or('0');
            }

            utils::Mode::Braille => {
                let r = utils::random_between(0x2840, 0x2840 + 63) as u32;
                self.symbol = std::char::from_u32(r).unwrap_or('0');
            }
        }
    }

    /// Rain
    pub fn rain(&mut self, rows: i32) {
        self.y = if self.y > rows {
            0
        } else {
            self.y + self.speed
        }
    }

    /// Render
    pub fn render(&mut self) {
        if self.y < 0 {
            return;
        }
        utils::cursor_move_to(self.y, self.x);
        print!("{}", utils::rgb(&self.symbol, self.color));
        if self.frame_count % self.switch_interval == 0 {
            self.set_symbol()
        }
        self.frame_count += 1;
    }
}
