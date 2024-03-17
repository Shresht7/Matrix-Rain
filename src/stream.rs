use crate::config;
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
}

impl Stream {
    /// Construct new stream
    pub fn new(x: i32, y: i32, config: &config::Config) -> Self {
        let mut s = Stream {
            entities: Vec::new(),
            x,
            y,
            speed: 1,
            count: utils::random_between(config.stream_min_count, config.stream_max_count),
        };
        s.generate_entities(config);
        return s;
    }

    /// Generate stream entities
    pub fn generate_entities(&mut self, config: &config::Config) {
        self.entities.clear();
        let mut y = self.y;
        for i in 0..self.count {
            let mut e = Entity::new(self.x, y, self.speed, i == 0, config);
            e.set_symbol();
            self.entities.push(e);
            y -= 1;
        }
    }

    /// Render stream
    pub fn render(&mut self, rows: i32, config: &config::Config) {
        for entity in self.entities.iter_mut() {
            entity.rain();
            entity.render();
        }

        match self.entities.last() {
            Some(e) => {
                e.clean(rows as u32);
                if e.y >= rows {
                    self.generate_entities(&config);
                }
            }
            _ => {}
        }
    }
}
