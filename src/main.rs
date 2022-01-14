use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, RgbImage};
use nalgebra_glm as glm;
use std::env;
use std::fs;
use std::iter::FromIterator;

const NUM_VOXELS: usize = 20;

struct CameraData {
    k: glm::Mat3,
    r: glm::Mat3,
    t: glm::Vec3,
}
impl CameraData {
    fn new(k: &[f32], r: &[f32], t: &[f32]) -> Self {
        CameraData {
            k: glm::mat3(k[0], k[1], k[2], k[3], k[4], k[5], k[6], k[7], k[8]),
            r: glm::mat3(r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], r[8]),
            t: glm::vec3(t[0], t[1], t[2]),
        }
    }
    fn projection_matrix(&self) -> glm::Mat3x4 {
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
}

struct View {
    camera: CameraData,
    img: Box<DynamicImage>,
}
impl View {
    fn new(camera: CameraData, img: DynamicImage) -> Self {
        View {
            camera,
            img: Box::new(img),
        }
    }
}

fn load_views() -> Vec<View> {
    let metadata_filename = "data/templeRing/templeR_par.txt";

    let metadata = fs::read_to_string(metadata_filename).expect("Couldn't read metadata file");

    let metadata = metadata
        .lines()
        .skip(1)
        .map(|line| {
            line.split(" ")
                .skip(1)
                .map(|n| n.parse::<f32>().unwrap())
                .collect::<Vec<f32>>()
        })
        .map(|line| CameraData::new(&line[0..9], &line[9..18], &line[18..21]));

    let images = (1..10)
        .map(|i| format!("data/templeRing/templeR{:0width$}.png", i, width = 4))
        .map(|filename| image::open(filename).expect("Couldn't open file"));

    let views: Vec<View> = metadata
        .zip(images)
        .map(|(camera, img)| View::new(camera, img))
        .collect();

    return views;
}

fn main() {
    let views = load_views();

    let voxels = [[[glm::vec3(); 3]; NUM_VOXELS]; NUM_VOXELS];
}
