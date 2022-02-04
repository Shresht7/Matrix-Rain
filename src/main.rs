mod entity;
mod stream;
mod utils;

//  ====
//  MAIN
//  ====

fn main() {
    //  Instantiate streams vector
    let mut streams: Vec<stream::Stream> = Vec::new();

    //  Generate stream entities
    for _i in 0..10 {
        let mut stream = stream::Stream::new();
        stream.generate_entities();
        streams.push(stream);
    }

    //  Render streams
    for stream in streams {
        println!("{:?}", stream.entities);
    }
}
