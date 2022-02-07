use crate::entity::Entity;
use crate::utils;

//  ======
//  STREAM
//  ======

pub struct Stream {
    /// The collection of entities that make up the stream
    pub entities: Vec<Entity>,

    //  Position
    x: i32,
    y: i32,

    /// Speed
    speed: i32,

    /// Count of entities
    count: u16,

    /// Stream entity color
    color: utils::RGBColor,

    /// Mode
    mode: utils::Mode,
}

impl Stream {
    /// Construct new stream
    pub fn new(
        x: i32,
        y: i32,
        min_count: u16,
        max_count: u16,
        color: utils::RGBColor,
        mode: utils::Mode,
    ) -> Self {
        let mut s = Stream {
            entities: Vec::new(),
            x,
            y,
            speed: utils::random_between(1, 3),
            count: utils::random_between(min_count, max_count),
            color,
            mode,
        };
        s.generate_entities();
        return s;
    }

    /// Generate stream entities
    pub fn generate_entities(&mut self) {
        self.entities.clear();
        let mut y = self.y;
        for i in 0..self.count {
            let mut e = Entity::new(self.x, y, self.speed, self.color.clone(), self.mode, i == 0);
            e.set_symbol();
            self.entities.push(e);
            y -= 1;
        }
    }

    /// Render stream
    pub fn render(&mut self, rows: i32) {
        for entity in self.entities.iter_mut() {
            entity.rain();
            entity.render();
        }

        match self.entities.last() {
            Some(e) => {
                e.clean(rows as u32);
                if e.y > rows {
                    self.generate_entities();
                }
            }
            _ => {}
        }
    }
}
