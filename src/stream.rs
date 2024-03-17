// Library
use crate::config;
use crate::entity::Entity;
use crate::utils;

//  ======
//  STREAM
//  ======

// Represents a single stream in the Matrix
pub struct Stream {
    /// The collection of entities that make up the stream
    pub entities: Vec<Entity>,

    // X Position
    x: i32,
    // Y Position
    y: i32,

    /// Speed
    speed: i32,

    /// Count of entities
    count: u16,
}

impl Stream {
    /// Construct new stream
    pub fn new(x: i32, y: i32, config: &config::Config) -> Self {
        let mut stream = Stream {
            entities: Vec::new(),
            x,
            y,
            speed: 1,
            count: utils::random_between(config.stream_min_count, config.stream_max_count),
        };
        stream.generate_entities(config);
        return stream;
    }

    /// Generate the entities that constitute the stream
    pub fn generate_entities(&mut self, config: &config::Config) {
        // Empty the entities vector
        self.entities.clear();

        // Create the leading entity
        self.entities
            .push(Entity::new_leader(self.x, self.y, self.speed, config));

        // Create the following entities
        for i in 1..self.count {
            let mut e = Entity::new(self.x, self.y - i as i32, self.speed, config);
            e.set_symbol();
            self.entities.push(e);
        }
    }

    /// Render stream
    pub fn render(&mut self, rows: i32, config: &config::Config) {
        // Move the stream down and render each entity
        for entity in self.entities.iter_mut() {
            entity.rain();
            entity.render();
        }

        // Clean up the last entity and regenerate when it's off the screen
        match self.entities.last() {
            Some(e) => {
                // Clean up the last entity
                e.clean(rows as u32);
                // Regenerate the stream and place it at the top if the last entity is off the screen
                // (i.e. the y position is greater than the number of rows).
                if e.y >= rows {
                    self.generate_entities(&config);
                }
            }
            _ => {}
        }
    }
}
