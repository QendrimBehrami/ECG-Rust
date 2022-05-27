use cgmath::{InnerSpace, Vector3};

use crate::terrain::Terrain;

pub mod diamond_square;
pub mod perlin;

///Initializ x and y coordinates in range of (-scale,scale)
fn init_grid(terrain: &mut crate::terrain::Terrain) {
    let (x_scale, y_scale, _) = terrain.scales;
    let size = terrain.size();
    for i in 0..size {
        for j in 0..size {
            terrain.vertices[(i, j)].x =
                (i as f32) / (size as f32 - 1.0) * 2f32 * x_scale - x_scale;
            terrain.vertices[(i, j)].y =
                (j as f32) / (size as f32 - 1.0) * 2f32 * y_scale - y_scale;
        }
    }
}

///Create normal vectors for given terrain
pub fn generate_normals(terrain: &mut Terrain) {
    let (x_scale, y_scale, z_scale) = terrain.scales;
    let size = terrain.size();
    let scaling_x = -z_scale / x_scale * size as f32;
    let scaling_y = -z_scale / y_scale * size as f32;
    for i in 0..size {
        for j in 0..size {
            let height = terrain.vertices[(i, j)].z;
            let dx;
            let dy;

            if i == 0 {
                dx = terrain.vertices[(i + 1, j)].z - height;
            } else if i == size - 1 {
                dx = height - terrain.vertices[(i - 1, j)].z;
            } else {
                dx = (terrain.vertices[(i + 1, j)].z - terrain.vertices[(i - 1, j)].z) / 2.0;
            }

            if j == 0 {
                dy = terrain.vertices[(i, j + 1)].z - height;
            } else if j == size - 1 {
                dy = height - terrain.vertices[(i, j - 1)].z;
            } else {
                dy = (terrain.vertices[(i, j + 1)].z - terrain.vertices[(i, j - 1)].z) / 2.0;
            }

            let normal = Vector3::new(dx * scaling_x, dy * scaling_y, 1.0).normalize();
            terrain.normals[(i, j)] = normal;
        }
    }
}

///Create texture coordinates in [0,1] space
pub fn generate_texels(terrain: &mut Terrain) {
    let size = terrain.size();
    for i in 0..size{
        for j in 0..size{
            terrain.texels[(i,j)].x = i as f32 / (size-1) as f32;
            terrain.texels[(i,j)].y = j as f32 / (size-1) as f32;
        }
    }
}
