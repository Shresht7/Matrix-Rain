use crossterm::cursor;
use crossterm::style::Print;
use crossterm::QueueableCommand;

use crate::config;
use crate::helpers::{colors, direction::Direction, utils};

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

        // Determine the speed based on the direction of motion
        let (speed_x, speed_y) = match config.direction {
            Direction::Down => (0.0, self.speed),
            Direction::Up => (0.0, -self.speed),
            Direction::Right => (self.speed, 0.0),
            Direction::Left => (-self.speed, 0.0),
            Direction::DiagonalRight => (self.speed, self.speed),
            Direction::DiagonalRightReverse => (-self.speed, -self.speed),
            Direction::DiagonalLeft => (-self.speed, self.speed),
            Direction::DiagonalLeftReverse => (self.speed, -self.speed),
        };

        // Create the leading entity
        self.entities.push(Entity::new(
            self.x,
            self.y,
            speed_x,
            speed_y,
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

            // Determine the entity starting x and y positions based on the direction of flow
            let (x, y) = match config.direction {
                Direction::Down => (self.x, self.y - i as f32),
                Direction::Up => (self.x, self.y + i as f32),
                Direction::Right => (self.x - i as f32, self.y),
                Direction::Left => (self.x + i as f32, self.y),
                Direction::DiagonalRight => (self.x - i as f32, self.y - i as f32),
                Direction::DiagonalRightReverse => (self.x + i as f32, self.y + i as f32),
                Direction::DiagonalLeft => (self.x + i as f32, self.y - i as f32),
                Direction::DiagonalLeftReverse => (self.x - i as f32, self.y + i as f32),
            };

            // Create the entity and add it to the entities vector
            let mut e = Entity::new(x, y, speed_x, speed_y, color, config);
            e.set_symbol();
            self.entities.push(e);
        }
    }

    /// Render the stream
    pub fn render(
        &mut self,
        rows: i32,
        columns: i32,
        config: &config::Config,
        stdout: &mut std::io::Stdout,
    ) -> std::io::Result<()> {
        // Check the last entity of the stream ...
        if let Some(e) = self.entities.last() {
            // Clean up the last entity. As the stream moves down, all entities will be overwritten
            // by the next frame, except for the trailing entity. So we manually overwrite it so that
            // the stream doesn't leave a trail.
            if !config.leave_trail {
                stdout
                    .queue(cursor::MoveTo(e.x as u16, e.y as u16))?
                    .queue(Print(" "))?;
            }

            // This is also a good time to check if the last entity is off the screen,
            // and if it is, we regenerate the stream and place it back at the top.
            let should_regenerate = match config.direction {
                Direction::Down => e.y >= rows as f32,
                Direction::Up => e.y < 0.0,
                Direction::Right => e.x >= columns as f32,
                Direction::Left => e.x < 0.0,
                Direction::DiagonalRight => e.x > columns as f32 && e.y > rows as f32,
                Direction::DiagonalRightReverse => e.x < 0.0 && e.y < 0.0,
                Direction::DiagonalLeft => e.x < 0.0 as f32 && e.y > rows as f32,
                Direction::DiagonalLeftReverse => e.x > columns as f32 && e.y > rows as f32,
            };

            if should_regenerate {
                self.generate_entities(config);
            }
        }

        // Move the stream down and render each entity
        for entity in self.entities.iter_mut() {
            entity.rain();
            entity.render(rows, columns, stdout)?;
        }

        Ok(())
    }
}
