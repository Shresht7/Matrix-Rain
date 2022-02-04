use crate::entity::Entity;
use crate::utils;

//  ======
//  STREAM
//  ======

pub struct Stream {
    /// The collection of entities that make up the stream
    pub entities: Vec<Entity>,

    /// The count of entities
    count: u8,

    /// Stream entity color
    color: utils::RGBColor,

    /// Speed offset
    speed_offset: u8,
}

impl Stream {
    /// Construct new stream
    pub fn new(min_count: u8, max_count: u8, color: utils::RGBColor) -> Self {
        Stream {
            entities: Vec::new(),
            count: utils::random_between(min_count, max_count),
            speed_offset: utils::random_between(0, 3),
            color,
        }
    }

    /// Generate stream entities
    pub fn generate_entities(&mut self, x: u8, mut y: u8) {
        for i in 0..self.count {
            let mut e = Entity::new(x, y, 1, self.color.clone(), i == 0);
            e.set_symbol();
            self.entities.push(e);
            y += 1;
        }
    }

    /// Render stream
    pub fn render(&mut self, rows: u8) {
        for entity in self.entities.iter_mut() {
            entity.render();
            entity.rain(rows, self.speed_offset);
        }
    }
}
