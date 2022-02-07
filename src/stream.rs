use crate::entity::Entity;
use crate::utils;

//  ======
//  STREAM
//  ======

pub struct Stream {
    /// The collection of entities that make up the stream
    pub entities: Vec<Entity>,

    /// Count of entities
    count: u16,

    /// Stream entity color
    color: utils::RGBColor,
}

impl Stream {
    /// Construct new stream
    pub fn new(min_count: u16, max_count: u16, color: utils::RGBColor) -> Self {
        Stream {
            entities: Vec::new(),
            count: utils::random_between(min_count, max_count),
            color,
        }
    }

    /// Generate stream entities
    pub fn generate_entities(&mut self, x: f32, mut y: f32, speed: f32, mode: utils::Mode) {
        for i in 0..self.count {
            let speed = (speed + utils::random_between(0.0, 10.0)) / 15.0;
            let mut e = Entity::new(x, y, speed, self.color.clone(), mode, i == 0);
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
