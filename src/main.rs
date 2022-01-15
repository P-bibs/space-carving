mod brdf;
mod ply;
mod volume;

use crate::volume::Volume;
use brdf::ConsistencyCheck;
use image::{DynamicImage, GenericImageView, Pixel};
use indicatif::ProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use nalgebra_glm as glm;
use std::fs;
use std::io;
use std::io::Write;

const NUM_IMAGES: usize = 40;

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
    fn translation(&self) -> glm::Vec3 {
        self.t
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

    let images = (1..NUM_IMAGES)
        .map(|i| format!("data/templeRing/templeR{:0width$}.png", i, width = 4))
        .map(|filename| image::open(filename).expect("Couldn't open file"))
        .progress();

    let views: Vec<View> = metadata
        .zip(images)
        .map(|(camera, img)| View::new(camera, img))
        .collect();

    return views;
}

fn carve_voxel(voxel: glm::IVec3, volume: &Volume, views: &Vec<&View>) -> bool {
    let position = volume.voxel_to_position(voxel.x as usize, voxel.y as usize, voxel.z as usize);

    let position = glm::vec4(
        position[0] as f32,
        position[2] as f32,
        position[1] as f32,
        1.0,
    );

    let mut colors_and_rays = vec![];

    for view in views {
        let width = view.img.width() as i32;
        let height = view.img.height() as i32;

        let back_projected: glm::Vec3 = view.camera.projection_matrix() * position;
        let back_projected = back_projected.xyz();

        let back_projected = glm::vec2(
            back_projected.x, // back_projected.z,
            back_projected.y, // back_projected.z,
        );

        let back_projected = glm::vec2(
            back_projected[0] + (width as f32 / 2.0),
            back_projected[1] + (height as f32 / 2.0),
        );

        // println!("Back projected: {:?}", back_projected);

        let x = back_projected.x.floor() as i32;
        let y = back_projected.y.floor() as i32;

        if x < 0 || x >= width || y < 0 || y >= height {
            // eprintln!("Back projected point is outside of image bounds");
            continue;
        }

        let pix = view.img.get_pixel(x as u32, y as u32);

        let scene_to_camera = view.camera.translation() - position.xyz();
        let color_vec = glm::vec3(
            pix.channels()[0] as f32 / 255.0,
            pix.channels()[1] as f32 / 255.0,
            pix.channels()[2] as f32 / 255.0,
        );

        colors_and_rays.push((color_vec, scene_to_camera));
    }

    if colors_and_rays.len() == 0 {
        return true;
    } else {
        let checker = brdf::VoxelColoring;

        let result = checker.consistent(&colors_and_rays);

        return result;
    }
}

fn main() {
    println!("Loading views");
    let views = load_views();
    println!("Views loaded");

    let voxel_size = 0.001;
    let front_top_left = glm::vec3(-0.023121, 0.121636, -0.017395);
    let back_bottom_right = glm::vec3(0.078626, -0.038009, -0.091940);
    let mut volume = Volume::new(voxel_size, front_top_left, back_bottom_right);

    let mut voxels_carved = 0;
    loop {
        println!("Carved {} voxels", voxels_carved);
        let mut converged = true;

        for plane_index in 0..volume.depth {
            let plane_in_world_space = volume.voxel_to_position(0, 0, plane_index).z;
            println!(
                "Carving plane {} at location {}",
                plane_index, plane_in_world_space
            );
            let non_occluded_views: Vec<_> = views
                .iter()
                .filter(|view| view.camera.translation()[2] > plane_in_world_space)
                .collect();
            println!("{} non_occluded_views", non_occluded_views.len());

            for y in 0..volume.height {
                // println!("Carving row {}", y);
                // io::stdout().flush();
                for x in 0..volume.width {
                    // println!("Carving voxel ({}, {})", x, y);
                    if *volume.get_voxel(x, y, plane_index) == false
                        || !volume.voxel_visible(x, y, plane_index)
                    {
                        continue;
                    }

                    let pos_voxel_space = glm::vec3(x as i32, y as i32, plane_index as i32);

                    let result = carve_voxel(pos_voxel_space, &volume, &non_occluded_views);

                    if result == false {
                        voxels_carved += 1;
                        *volume.get_voxel(x, y, plane_index) = false;
                        converged = false;
                    }
                }
            }
        }

        if converged {
            break;
        }
    }

    ply::write_ply(&mut volume, "out.ply");

    println!("Finished. Carved {} voxels", voxels_carved);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_voxel_getters() {
        let mut volume = Volume::new(0.5, glm::vec3(-1.5, 1.5, 1.5), glm::vec3(1.5, -1.5, -1.5));
        *volume.get_voxel_ws(-1.4, 1.4, 1.4) = true;
        assert_eq!(*volume.get_voxel_ws(-1.1, 1.1, 1.1), true);

        *volume.get_voxel_ws(0.0, 0.0, 0.0) = true;
        assert_eq!(*volume.get_voxel_ws(0.0, 0.0, 0.0), true);
    }
    #[test]
    fn test_voxel_to_world_space_converter() {
        let volume = Volume::new(1., glm::vec3(0., 2., 0.), glm::vec3(2., 0., -2.));
        assert_eq!(volume.voxel_to_position(0, 0, 0), glm::vec3(0.5, 1.5, -0.5));

        let volume = Volume::new(0.5, glm::vec3(0., 2., 0.), glm::vec3(2., 0., -2.));
        assert_eq!(
            volume.voxel_to_position(0, 0, 0),
            glm::vec3(0.25, 1.75, -0.25)
        );
        assert_eq!(
            volume.voxel_to_position(1, 1, 1),
            glm::vec3(0.75, 1.25, -0.75)
        );

        let volume = Volume::new(0.5, glm::vec3(0., 2., -1.), glm::vec3(2., 0., -2.));
        assert_eq!(
            volume.voxel_to_position(0, 0, 0),
            glm::vec3(0.25, 1.75, -1.25)
        );
        assert_eq!(
            volume.voxel_to_position(1, 1, 1),
            glm::vec3(0.75, 1.25, -1.75)
        );
    }
}
