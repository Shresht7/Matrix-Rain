use crate::utils::RGBColor;

//  ======
//  ENTITY
//  ======

#[derive(Debug)]
pub struct Entity {
    pub x: i32,
    pub speed: i32,
    pub color: RGBColor,
}
