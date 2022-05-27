use cgmath::{Vector2, Vector3, Zero};
use array2d::Array2D;

///Representation of a terrain as combination of vertices, normals and texels
#[derive(Debug)]
pub struct Terrain {
    ///The width and height for a quadratic terrain
    size: usize,
    pub scales: (f32,f32,f32),
    pub vertices: Array2D<Vector3<f32>>,
    pub normals: Array2D<Vector3<f32>>,
    pub texels: Array2D<Vector2<f32>>,
}

impl Terrain {
    pub fn new(size: usize,scales : (f32,f32,f32)) -> Self {
        Self {
            size,
            scales,
            vertices: Array2D::filled_with(
                Vector3::zero(),
                size,
                size,
            ),
            normals: Array2D::filled_with(
                Vector3::zero(),
                size,
                size,
            ),
            texels: Array2D::filled_with(Vector2::zero(), size, size),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
