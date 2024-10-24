use crossterm::cursor;
use crossterm::style::Print;
use crossterm::QueueableCommand;

use crate::config;
use crate::helpers::{colors, utils};

use super::entity::Entity;

//  ======
//  STREAM
//  ======

// Represents a single stream in the Matrix
pub struct Stream {
    /// The collection of [entities](Entity) that make up the stream
    pub entities: Vec<Entity>,

    /// X Position
    x: f32,
    /// Y Position
    y: f32,

    /// Speed
    speed: f32,

    /// Count of [entities](Entity) in the stream
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
        let gradient = colors::LinearGradient::new(
            colors::RGBColor::from(config.stream_color),
            colors::RGBColor::from(config.stream_color) * config.stream_color_gradient_factor, // Overloaded Operator for Scalar Multiplication
        );

        // Create the following entities
        for i in 1..self.count {
            // Determine the color of the entity based on the gradient
            let color = gradient.interpolate(i as f32 / self.count as f32);

            // Create the entity and add it to the entities vector
            let mut e = Entity::new(self.x, self.y - i as f32, self.speed, color, config);
            e.set_symbol();
            self.entities.push(e);
        }
    }

    /// Render the stream
    pub fn render(
        &mut self,
        rows: i32,
        config: &config::Config,
        stdout: &mut std::io::Stdout,
    ) -> std::io::Result<()> {
        // Check the last entity of the stream ...
        if let Some(e) = self.entities.last() {
            // Clean up the last entity. As the stream moves down, all entities will be overwritten
            // by the next frame, except for the trailing entity. So we manually overwrite it so that
            // the stream doesn't leave a trail.
            stdout
                .queue(cursor::MoveTo(e.x as u16, e.y as u16))?
                .queue(Print(" "))?;

            // This is also a good time to check if the last entity is off screen,
            // (i.e. the y position is greater than the number of rows)
            // and if it is, we regenerate the stream and place it back at the top.
            if e.y >= rows as f32 {
                self.generate_entities(&config);
            }
        }

        // Move the stream down and render each entity
        for entity in self.entities.iter_mut() {
            entity.rain();
            entity.render(stdout)?;
        }

        Ok(())
    }
}
