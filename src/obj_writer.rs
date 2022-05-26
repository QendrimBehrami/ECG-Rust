use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::terrain::Terrain;

pub fn create_obj(file_name: &str, terrain: &Terrain) {
    let file_name = format!("{}.obj", file_name);
    let file =
        File::create(&file_name).expect(&format!("{} {}", "Failed to open file:", &file_name));
    let mut writer = BufWriter::new(file);
    let size = terrain.size();

    //Write vertex data
    terrain.vertices.elements_row_major_iter().for_each(|v| {
        writer
            .write(format!("v {} {} {}\n", v.x, v.y, v.z).as_bytes())
            .expect("Failed to write vertex values");
    });

    //Generate faces
    for i in 1..size {
        for j in 1..size {
            writer
                .write(
                    format!(
                        "f {} {} {}\n",
                        i * size + j + 1,       //Self
                        (i - 1) * size + j,     //Self top left
                        (i - 1) * size + j + 1  //Self top
                    )
                    .as_bytes(),
                )
                .expect("Failed to write faces!");
            writer
                .write(
                    format!(
                        "f {} {} {}\n",
                        i * size + j + 1,   //Self
                        (i - 1) * size + j, //Self top left
                        i * size + j        //Self left
                    )
                    .as_bytes(),
                )
                .expect("Failed to write faces!");
        }
    }
}

pub fn create_normal_map(file_name: &str, terrain: &Terrain) {
    let file_name = format!("{}.ppm", file_name);
    let file =
        File::create(&file_name).expect(&format!("{} {}", "Failed to open file:", &file_name));
    let mut writer = BufWriter::new(file);
    let size = terrain.size();
    let header = format!("P3\n{} {}\n255\n", size, size);
    writer
        .write(header.as_bytes())
        .expect("Failed to write to normalmap");
    for i in 0..size {
        for j in 0..size {
            let normal = &terrain.normals[(i, j)];
            let nx = (normal.x * 127.0) as u8 + 127;
            let ny = (normal.y * 127.0) as u8 + 127;
            let nz = (normal.z * 127.0) as u8 + 127;
            writer
                .write(format!("{} {} {} ", nx, ny, nz).as_bytes())
                .expect("Failed to write normal values");
        }
        writer
            .write("\n".as_bytes())
            .expect("Failed to write normal values");
    }
}
