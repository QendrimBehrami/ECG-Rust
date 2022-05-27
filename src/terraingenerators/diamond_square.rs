// use cgmath::{InnerSpace, Vector3};
use rand::Rng;
use rand_distr::Normal;

use crate::terrain::Terrain;

#[allow(dead_code)]
pub fn generate_terrain(terrain: &mut Terrain, displacement: f32, roughness: f32) {
    let size = terrain.size();
    let z_scale = terrain.scales.2;
    assert!(
        (size - 1) & (size - 2) == 0,
        "Terrain size is not power of 2 plus 1"
    );
    //Initialize x and y coordinates
    super::init_grid(terrain);
    //Initialize uniform distribution
    let distribution = rand::distributions::Uniform::new(-z_scale,z_scale);
    let mut generator = rand::thread_rng();

    //Assign random values to corners
    terrain.vertices[(0, 0)].z = generator.sample(distribution);
    terrain.vertices[(0, size - 1)].z = generator.sample(distribution);
    terrain.vertices[(size - 1, 0)].z = generator.sample(distribution);
    terrain.vertices[(0, size - 1)].z = generator.sample(distribution);

    let mut scaling: f32 = z_scale;
    let mut step_size: usize = size / 2;
    let normal = Normal::new(0.0, displacement * z_scale).unwrap();

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
