use crate::terrain::Terrain;
use cgmath::{InnerSpace, Vector2};

use super::init_grid;

///Original Perlin hash table
const PERMUTATIONS: &'static [u8] = &[
    51, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

pub fn generate_terrain(
    terrain: &mut Terrain,
    frequency: f32,
    amplitude: f32,
    iterations: u8,
    scales: (f32, f32, f32),
) {
    //Map into scales
    let size = terrain.size();
    init_grid(terrain, scales);

    //Use displacement as frequency and roughness as amplitude

    for i in 0..size {
        for j in 0..size {
            let mut height = 0.0;
            let mut amplitude = amplitude;
            let mut frequency = frequency;
            for _ in 0..iterations {
                height += scales.2 * amplitude * perlin(i as f32 * frequency, j as f32 * frequency);
                amplitude *= 0.5;
                frequency *= 2.0;
            }
            terrain.vertices[(i, j)].z = height;
        }
    }
}

fn perlin(x: f32, y: f32) -> f32 {
    let xi: usize = x.floor() as usize % 256;
    let yi: usize = y.floor() as usize % 256;
    let xf: f32 = x - x.floor();
    let yf: f32 = y - y.floor();

    let top_right = Vector2::new(xf - 1.0, yf - 1.0);
    let top_left = Vector2::new(xf, yf - 1.0);
    let bottom_right = Vector2::new(xf - 1.0, yf);
    let bottom_left = Vector2::new(xf, yf);

    // if xi == 255 || yi == 255{
    //     println!("HIER");
    // }

    let value_top_right = PERMUTATIONS[(PERMUTATIONS[xi + 1] as usize + yi + 1) % 256];
    let value_top_left = PERMUTATIONS[(PERMUTATIONS[xi] as usize + yi + 1) % 256];
    let value_bottom_right = PERMUTATIONS[(PERMUTATIONS[xi + 1] as usize + yi) % 256];
    let value_bottom_left = PERMUTATIONS[(PERMUTATIONS[xi] as usize + yi) % 256];

    let dot_top_right = top_right.dot(get_constant_vector(value_top_right));
    let dot_top_left = top_left.dot(get_constant_vector(value_top_left));
    let dot_bottom_right = bottom_right.dot(get_constant_vector(value_bottom_right));
    let dot_bottom_left = bottom_left.dot(get_constant_vector(value_bottom_left));

    let u = fade(xf);
    let v = fade(yf);

    linear_interpolate(
        u,
        linear_interpolate(v, dot_bottom_left, dot_top_left),
        linear_interpolate(v, dot_bottom_right, dot_top_right),
    )
}

fn get_constant_vector(value: u8) -> Vector2<f32> {
    match value % 4 {
        0 => Vector2::new(1.0, 1.0),
        1 => Vector2::new(-1.0, 1.0),
        2 => Vector2::new(-1.0, -1.0),
        3 => Vector2::new(1.0, -1.0),
        _ => unreachable!(),
    }
}

fn linear_interpolate(factor: f32, a: f32, b: f32) -> f32 {
    a + factor * (b - a)
}

fn fade(value: f32) -> f32 {
    ((6.0 * value - 15.0) * value + 10.0) * value * value * value
}
