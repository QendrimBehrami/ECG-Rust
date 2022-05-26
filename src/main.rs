mod obj_writer;
mod terrain;
mod terraingenerators;

use std::{fs::File, io::Read};

use serde::Deserialize;
use terrain::Terrain;
use terraingenerators::{diamond_square, perlin};

#[derive(Debug, Deserialize)]
struct Config {
    pub size: usize,
    pub frequency: f32,
    pub amplitude: f32,
    pub iterations: u8,
    pub scales: (f32, f32, f32),
    pub file_name: String,
    
}

impl Config {
    pub fn read_from_file(file_name: &str) -> Self {
        let mut file = File::open(file_name).expect("Failed to read config file");
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        serde_yaml::from_str(content.as_str()).expect("Failed to parse config")
    }
}

fn main() {
    let config = Config::read_from_file("config.yml");
    let mut terrain = Terrain::new(config.size);
    // diamond_square::generate_terrain(
    //     &mut terrain,
    //     config.displacement,
    //     config.roughness,
    //     config.scales,
    // );
    perlin::generate_terrain(&mut terrain,config.frequency, config.amplitude,config.iterations, config.scales);
    diamond_square::generate_normals(&mut terrain, config.scales);
    obj_writer::create_obj(config.file_name.as_str(), &terrain);
    obj_writer::create_normal_map(config.file_name.as_str(), &terrain);
}
