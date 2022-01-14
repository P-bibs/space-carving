use image::{DynamicImage, GenericImageView};
use nalgebra_glm as glm;
use std::env;
use std::fs;

const NUM_IMAGES: usize = 10;

struct Volume {
    data: Vec<Vec<Vec<bool>>>,
    voxel_size: f32,
    width: usize,
    height: usize,
    depth: usize,
}
impl Volume {
    fn new(voxel_size: f32, front_top_left: glm::Vec3, back_bottom_right: glm::Vec3) -> Self {
        let width = ((back_bottom_right.x - front_top_left.x).abs() / voxel_size).ceil() as usize;
        let height = ((back_bottom_right.y - front_top_left.y).abs() / voxel_size).ceil() as usize;
        let depth = ((back_bottom_right.z - front_top_left.z).abs() / voxel_size).ceil() as usize;

        // Ensure coordinates are even so the origin doesn't fall between voxels
        let width = if width % 2 == 1 { width + 1 } else { width };
        let height = if height % 2 == 1 { height + 1 } else { height };
        let depth = if depth % 2 == 1 { depth + 1 } else { depth };

        let mut cols = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                let mut depth_line = vec![];
                for _ in 0..depth {
                    depth_line.push(false);
                }
                row.push(depth_line);
            }
            cols.push(row);
        }

        debug_assert_eq!(cols.len(), height);
        debug_assert_eq!(cols[0].len(), width);
        debug_assert_eq!(cols[0][0].len(), depth);

        println!(
            "Created volume with dimensions: {}x{}x{}",
            width, height, depth
        );

        Self {
            data: cols,
            voxel_size,
            width: width as usize,
            height: height as usize,
            depth: depth as usize,
        }
    }
    fn get_voxel(&self, x: usize, y: usize, z: usize) -> bool {
        self.data[y][x][z]
    }
    fn get_voxel_ws(&mut self, x: f32, y: f32, z: f32) -> &mut bool {
        print!("Converting index at ({}, {}, {}) to voxel index: ", x, y, z);

        // Flip y and z to match the 3d array coordinate system (origin in front-top-left)
        let x = x;
        let y = -y;
        let z = -z;

        // Since the voxels are not unit size, we need to scale the coordinates
        // to the correct voxel.
        let x = (x / self.voxel_size);
        let y = (y / self.voxel_size);
        let z = (z / self.voxel_size);

        // Next, we need to shift by the values from being centered around 0 to
        // only the positive side of each axis
        let x = x + (self.width as f32 / 2.0);
        let y = y + (self.height as f32 / 2.0);
        let z = z + (self.depth as f32 / 2.0);

        // floor floats to ints
        let x = x.floor() as usize;
        let y = y.floor() as usize;
        let z = z.floor() as usize;

        println!("{}, {}, {}", x, y, z);
        &mut self.data[y][x][z]
    }
}

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

    let volume = Volume::new(
        0.1,
        glm::vec3(-0.023121, -0.038009, -0.091940),
        glm::vec3(0.078626, 0.121636, -0.017395),
    );
}

#[test]
fn test_volume_methods() {
    let mut volume = Volume::new(0.5, glm::vec3(-1.5, 1.5, 1.5), glm::vec3(1.5, -1.5, -1.5));
    *volume.get_voxel_ws(-1.4, 1.4, 1.4) = true;
    assert_eq!(*volume.get_voxel_ws(-1.1, 1.1, 1.1), true);

    *volume.get_voxel_ws(0.0, 0.0, 0.0) = true;
    assert_eq!(*volume.get_voxel_ws(0.0, 0.0, 0.0), true);
}
