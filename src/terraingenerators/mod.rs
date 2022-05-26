pub mod diamond_square;
pub mod perlin;


///Initializ x and y coordinates in range of (-scale,scale)
fn init_grid(terrain: &mut crate::terrain::Terrain, scales: (f32, f32, f32)) {
    let (x_scale, y_scale, _) = scales;
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
