use crate::brdf;
use crate::brdf::ConsistencyCheck;
use crate::view::View;
use crate::volume::Volume;
use image::{GenericImageView, Pixel};
use nalgebra_glm as glm;

pub fn carve_voxel(voxel: glm::IVec3, volume: &Volume, views: &mut Vec<&mut View>) -> bool {
    let position = volume.voxel_to_position(voxel.x as usize, voxel.y as usize, voxel.z as usize);

    let position = glm::vec4(
        position[0] as f32,
        position[2] as f32,
        position[1] as f32,
        1.0,
    );

    let mut colors_and_rays = vec![];
    let mut masks = vec![];

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

        if view.mask[y as usize][x as usize] {
            // this point has already been used to correlate and therefore there's another voxel occluding
            continue;
        }

        let pix = view.img.get_pixel(x as u32, y as u32);

        let scene_to_camera = view.camera.translation() - position.xyz();
        let color_vec = glm::vec3(
            pix.channels()[0] as f32 / 255.0,
            pix.channels()[1] as f32 / 255.0,
            pix.channels()[2] as f32 / 255.0,
        );

        let mask_value = view
            .mask
            .get_mut(y as usize)
            .unwrap()
            .get_mut(x as usize)
            .unwrap();

        masks.push(mask_value);
        colors_and_rays.push((color_vec, scene_to_camera));
    }

    if colors_and_rays.len() == 0 {
        return true;
    } else {
        let checker = brdf::VoxelColoring;

        let result = checker.consistent(&colors_and_rays);

        if result {
            for mask in masks {
                *mask = true;
            }
        }

        return result;
    }
}
