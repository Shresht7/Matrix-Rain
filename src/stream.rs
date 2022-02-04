use crate::entity::Entity;
use crate::utils::RGBColor;

//  ======
//  STREAM
//  ======

pub struct Stream {
    pub entities: Vec<Entity>,
}

impl Stream {
    pub fn new() -> Self {
        Stream {
            entities: Vec::new(),
        }
    }

    pub fn generate_entities(&mut self) {
        for i in 0..5 {
            self.entities.push(Entity {
                x: i,
                speed: 1,
                color: RGBColor(100, 255, 100),
            });
        }
    }
}
