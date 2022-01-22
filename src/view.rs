use image::{DynamicImage, GenericImageView};
use nalgebra_glm as glm;

pub struct CameraData {
    k: glm::Mat3,
    r: glm::Mat3,
    t: glm::Vec3,
}
impl CameraData {
    pub fn new(k: &[f32], r: &[f32], t: &[f32]) -> Self {
        CameraData {
            k: glm::mat3(k[0], k[1], k[2], k[3], k[4], k[5], k[6], k[7], k[8]),
            r: glm::mat3(r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], r[8]),
            t: glm::vec3(t[0], t[1], t[2]),
        }
    }
    pub fn projection_matrix(&self) -> glm::Mat3x4 {
        let rt = glm::mat3x4(
            self.r[(0, 0)],
            self.r[(0, 1)],
            self.r[(0, 2)],
            self.t[0],
            self.r[(1, 0)],
            self.r[(1, 1)],
            self.r[(1, 2)],
            self.t[1],
            self.r[(2, 0)],
            self.r[(2, 1)],
            self.r[(2, 2)],
            self.t[2],
        );
        self.k * rt
    }
    pub fn translation(&self) -> glm::Vec3 {
        self.t
    }
}

pub struct View {
    pub camera: CameraData,
    pub img: Box<DynamicImage>,
    pub mask: Vec<Vec<bool>>,
}
impl View {
    pub fn new(camera: CameraData, img: DynamicImage) -> Self {
        let mut mask = vec![];
        for _ in 0..img.height() {
            mask.push(vec![false; img.width() as usize]);
        }

        View {
            camera,
            img: Box::new(img),
            mask,
        }
    }
    /// reset the mask to all false
    pub fn reset_mask(&mut self) {
        for row in &mut self.mask {
            for cell in row {
                *cell = false;
            }
        }
    }
}
