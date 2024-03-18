// Library
use crate::ansi;
use crate::config;
use crate::entity::Entity;
use crate::utils;
use colorgrad::{self, Color};

//  ======
//  STREAM
//  ======

// Represents a single stream in the Matrix
pub struct Stream {
    /// The collection of entities that make up the stream
    pub entities: Vec<Entity>,

    // X Position
    x: f32,
    // Y Position
    y: f32,

    /// Speed
    speed: f32,

    /// Count of entities
    count: u16,
}

impl Stream {
    /// Construct new stream
    pub fn new(x: f32, y: f32, config: &config::Config) -> Self {
        let mut stream = Stream {
            entities: Vec::new(),
            x,
            y,
            speed: 1.0,
            count: 10,
        };
        stream.generate_entities(config);
        return stream;
    }

    /// Generate the entities that constitute the stream
    pub fn generate_entities(&mut self, config: &config::Config) {
        // Empty the entities vector
        self.entities.clear();

        // Randomize the speed
        self.speed = utils::random_between(0.125, 1.0);

        // Randomize the count
        self.count = utils::random_between(config.stream_min_count, config.stream_max_count);

        // Create the leading entity
        self.entities.push(Entity::new(
            self.x,
            self.y,
            self.speed,
            config.leading_entity_color,
            config,
        ));

        // Create the color gradient for the stream
        let gradient = colorgrad::CustomGradient::new()
            .colors(&[
                Color::from_rgba8(
                    config.stream_color.0,
                    config.stream_color.1,
                    config.stream_color.2,
                    255,
                ),
                Color::from_rgba8(
                    (config.stream_color.0 as f32 * 0.33).floor() as u8,
                    (config.stream_color.1 as f32 * 0.33).floor() as u8,
                    (config.stream_color.2 as f32 * 0.33).floor() as u8,
                    255,
                ),
            ])
            .build()
            .unwrap();

        // Create the following entities
        for i in 1..self.count {
            // Determine the color of the entity based on the gradient
            let color = {
                let [r, g, b, _] = gradient.at(i as f64 / self.count as f64).to_rgba8();
                ansi::RGBColor(r, g, b)
            };

            // Create the entity and add it to the entities vector
            let mut e = Entity::new(self.x, self.y - i as f32, self.speed, color, config);
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
                if e.y >= rows as f32 {
                    self.generate_entities(&config);
                }
            }
            _ => {}
        }
    }
}
