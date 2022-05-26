use cgmath::{InnerSpace, Vector3};
use rand::Rng;
use rand_distr::Normal;

use crate::terrain::Terrain;

#[allow(dead_code)]
pub fn generate_terrain(
    terrain: &mut Terrain,
    displacement: f32,
    roughness: f32,
    scales: (f32, f32, f32),
) {
    let size = terrain.size();
    assert!(
        (size - 1) & (size - 2) == 0,
        "Terrain size is not power of 2 plus 1"
    );
    //Initialize x and y coordinates
    super::init_grid(terrain, scales);
    //Initialize uniform distribution
    let distribution = rand::distributions::Uniform::new(-scales.2, scales.2);
    let mut generator = rand::thread_rng();

    //Assign random values to corners
    terrain.vertices[(0, 0)].z = generator.sample(distribution);
    terrain.vertices[(0, size - 1)].z = generator.sample(distribution);
    terrain.vertices[(size - 1, 0)].z = generator.sample(distribution);
    terrain.vertices[(0, size - 1)].z = generator.sample(distribution);

    let mut scaling: f32 = scales.2;
    let mut step_size: usize = size / 2;
    let normal = Normal::new(0.0, displacement * scales.2).unwrap();

    while step_size > 0 {
        for i in (step_size..size).step_by(2 * step_size) {
            for j in (step_size..size).step_by(2 * step_size) {
                diamond_step(terrain, (i, j), step_size, scaling, normal);
                square_step(terrain, (i - step_size, j), step_size, scaling, normal);
                square_step(terrain, (i + step_size, j), step_size, scaling, normal);
                square_step(terrain, (i, j - step_size), step_size, scaling, normal);
                square_step(terrain, (i, j + step_size), step_size, scaling, normal);
            }
        }
        step_size /= 2;
        if roughness == 1.0 {
            scaling /= 2.0;
        } else {
            scaling /= 2.0f32.powf(roughness);
        }
    }
}

fn diamond_step(
    terrain: &mut Terrain,
    (i, j): (usize, usize),
    step_size: usize,
    scale: f32,
    distribution: Normal<f32>,
) {
    let height1 = terrain.vertices[(i - step_size, j - step_size)].z;
    let height2 = terrain.vertices[(i + step_size, j - step_size)].z;
    let height3 = terrain.vertices[(i - step_size, j + step_size)].z;
    let height4 = terrain.vertices[(i + step_size, j + step_size)].z;
    let average = 0.25 * (height1 + height2 + height3 + height4);
    let height = average + rand::thread_rng().sample(distribution) * scale;
    terrain.vertices[(i, j)].z = height;
}

fn square_step(
    terrain: &mut Terrain,
    (i, j): (usize, usize),
    step_size: usize,
    scale: f32,
    distribution: Normal<f32>,
) {
    let mut total = 0;
    let mut sum = 0.0;
    let size = terrain.size();

    //Check if not on left border
    if i != 0 {
        total += 1;
        sum += terrain.vertices[(i - step_size, j)].z;
    }

    //Check if not on right border
    if i != size - 1 {
        total += 1;
        sum += terrain.vertices[(i + step_size, j)].z;
    }

    //Check if not on top border
    if j != 0 {
        total += 1;
        sum += terrain.vertices[(i, j - step_size)].z;
    }

    //Check if not on bottom border
    if j != size - 1 {
        total += 1;
        sum += terrain.vertices[(i, j + step_size)].z;
    }

    let mut height = sum / total as f32;
    height += rand::thread_rng().sample(distribution) * scale;
    terrain.vertices[(i, j)].z = height;
}

pub fn generate_normals(terrain: &mut Terrain, scales: (f32, f32, f32)) {
    let (x_scale, y_scale, z_scale) = scales;
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

            //TODO dy
            let normal = Vector3::new(dx * scaling_x, dy * scaling_y, 1.0).normalize();
            terrain.normals[(i, j)] = normal;
        }
    }
}
