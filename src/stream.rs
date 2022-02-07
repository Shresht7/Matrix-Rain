use crate::entity::Entity;
use crate::utils;

//  ======
//  STREAM
//  ======

pub struct Stream {
    /// The collection of entities that make up the stream
    pub entities: Vec<Entity>,

    //  Position
    x: f32,
    y: f32,

    /// Speed
    speed: f32,

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
        x: f32,
        y: f32,
        min_count: u16,
        max_count: u16,
        color: utils::RGBColor,
        mode: utils::Mode,
    ) -> Self {
        let mut s = Stream {
            entities: Vec::new(),
            x,
            y,
            speed: utils::random_between(1.0, 10.0) / 5.0,
            count: utils::random_between(min_count, max_count),
            color,
            mode,
        };
        s.generate_entities();
        return s;
    }

    /// Generate stream entities
    pub fn generate_entities(&mut self) {
        let mut y = self.y;
        for i in 0..self.count {
            let mut e = Entity::new(self.x, y, self.speed, self.color.clone(), self.mode, i == 0);
            e.set_symbol();
            self.entities.push(e);
            y -= 1.0;
        }
    }

    /// Render stream
    pub fn render(&mut self, rows: i32) {
        for entity in self.entities.iter_mut() {
            entity.rain(rows);
            entity.render();
        }
        match self.entities.last() {
            Some(e) => e.clean(rows as u32),
            _ => {}
        }
    }
}
