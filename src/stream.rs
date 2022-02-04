use crate::entity::Entity;
use crate::utils;

//  ======
//  STREAM
//  ======

pub struct Stream {
    /// The collection of entities that make up the stream
    pub entities: Vec<Entity>,

    /// Count of entities
    count: i32,

    /// Stream entity color
    color: utils::RGBColor,
}

impl Stream {
    /// Construct new stream
    pub fn new(min_count: i32, max_count: i32, color: utils::RGBColor) -> Self {
        Stream {
            entities: Vec::new(),
            count: utils::random_between(min_count, max_count),
            color,
        }
    }

    /// Generate stream entities
    pub fn generate_entities(&mut self, x: i32, mut y: i32, speed: i32, mode: utils::Mode) {
        for i in 0..self.count {
            let speed = (speed + utils::random_between(0, 10)) / 15;
            let mut e = Entity::new(x, y, speed, self.color.clone(), mode, i == 0);
            e.set_symbol();
            self.entities.push(e);
            y -= 1;
        }
    }

    /// Render stream
    pub fn render(&mut self, rows: i32) {
        for entity in self.entities.iter_mut() {
            entity.render();
            entity.rain(rows);
        }
        match self.entities.last() {
            Some(e) => e.clean(),
            _ => {}
        }
    }
}
