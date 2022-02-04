//  ======
//  ENTITY
//  ======

#[derive(Debug)]
struct RGBColor(u8, u8, u8);

#[derive(Debug)]
struct Entity {
    x: i32,
    speed: i32,
    color: RGBColor,
}

//  ======
//  STREAM
//  ======

struct Stream {
    entities: Vec<Entity>,
}

impl Stream {
    fn new() -> Self {
        Stream {
            entities: Vec::new(),
        }
    }

    fn generate_entities(&mut self) {
        for i in 0..5 {
            self.entities.push(Entity {
                x: i,
                speed: 1,
                color: RGBColor(100, 255, 100),
            });
        }
    }
}

//  ====
//  MAIN
//  ====

fn main() {
    //  Instantiate streams vector
    let mut streams: Vec<Stream> = Vec::new();

    //  Generate stream entities
    for _i in 0..10 {
        let mut stream = Stream::new();
        stream.generate_entities();
        streams.push(stream);
    }

    //  Render streams
    for stream in streams {
        println!("{:?}", stream.entities);
    }
}
