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

    //Write normals
    terrain.normals.elements_row_major_iter().for_each(|n| {
        writer
            .write(format!("vn {} {} {}\n", n.x, n.y, n.z).as_bytes())
            .expect("Failed to write normal values");
    });

    //Write texels
    terrain.texels.elements_row_major_iter().for_each(|t| {
        writer
            .write(format!("vt {} {}\n", t.x, t.y).as_bytes())
            .expect("Failed to write texel values");
    });

    //Generate faces
    for i in 1..size {
        for j in 1..size {
            let current_index = i * size + j + 1;
            let left_index = i * size + j;
            let top_index = (i - 1) * size + j + 1;
            let top_left_index = (i - 1) * size + j;
            writer
                .write(
                    format!(
                        "f {}/{}/{} {}/{}/{} {}/{}/{}\n",
                        current_index,
                        current_index,
                        current_index,
                        top_index,
                        top_index,
                        top_index,
                        top_left_index,
                        top_left_index,
                        top_left_index
                    )
                    .as_bytes(),
                )
                .expect("Failed to write faces!");
            writer
                .write(
                    format!(
                        "f {}/{}/{} {}/{}/{} {}/{}/{}\n",
                        current_index,
                        current_index,
                        current_index,
                        left_index,
                        left_index,
                        left_index,
                        top_left_index,
                        top_left_index,
                        top_left_index
                    )
                    .as_bytes(),
                )
                .expect("Failed to write faces!");
            // writer
            //     .write(
            //         format!("f {} {} {}\n", current_index, left_index, top_left_index).as_bytes(),
            //     )
            //     .expect("Failed to write faces!");
        }
    }
}

pub fn create_normal_map(file_name: &str, terrain: &Terrain) {
    let file_name = format!("{}-normalmap.ppm", file_name);
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

pub fn create_height_map(file_name: &str, terrain: &Terrain) {
    let file_name = format!("{}-heightmap.pgm", file_name);
    let file =
        File::create(&file_name).expect(&format!("{} {}", "Failed to open file:", &file_name));
    let mut writer = BufWriter::new(file);
    let size = terrain.size();
    let header = format!("P2\n{} {}\n255\n", size, size);
    writer
        .write(header.as_bytes())
        .expect("Failed to write to heightmap");
    for i in 0..size {
        for j in 0..size {
            let height = (&terrain.vertices[(i, j)].z * 127.0 / terrain.scales.2) as u8 + 127;
            writer
                .write(format!("{} ", height).as_bytes())
                .expect("Failed to write height values");
        }
        writer
            .write("\n".as_bytes())
            .expect("Failed to write height values");
    }
}
