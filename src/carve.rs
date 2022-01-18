use crate::brdf;
use crate::brdf::ConsistencyCheck;
use crate::exporter;
use crate::view::View;
use crate::volume::{Color, Volume, Voxel};
use image::{GenericImage, GenericImageView, Pixel};
use nalgebra_glm as glm;

pub fn carve_voxel(
    voxel: glm::IVec3,
    volume: &Volume,
    views: &mut Vec<&mut View>,
) -> Option<Color> {
    // Convert voxel-space coordinates to scene-space
    let position = volume.voxel_to_position(voxel.x as usize, voxel.y as usize, voxel.z as usize);

    // Convert to homogenous coordinates
    let position = glm::vec4(
        position[0] as f32,
        position[1] as f32,
        position[2] as f32,
        1.0,
    );

    let mut colors_and_rays = vec![];
    let mut masks = vec![];

    for view in views {
        let width = view.img.width() as i32;
        let height = view.img.height() as i32;

        // Back project scene element onto image
        let back_projected: glm::Vec3 = view.camera.projection_matrix() * position;

        // Scale by `z` to account for back projection ambiguity
        let back_projected = glm::vec2(
            back_projected.x / back_projected.z,
            back_projected.y / back_projected.z,
        );

        let x = back_projected.x.floor() as i32;
        let y = back_projected.y.floor() as i32;

        // Skip view if back projected point falls outside captured image
        if x < 0 || x >= width || y < 0 || y >= height {
            // eprintln!("Back projected point is outside of image bounds");
            continue;
        }

        // If this pixel of the image has already been matched to a scene
        // element, then that element occludes this new element so we
        // should skip it
        if view.mask[y as usize][x as usize] {
            continue;
        }

        let pix = view.img.get_pixel(x as u32, y as u32);

        // calculate the vector from the scene voxel to the camera
        let scene_to_camera = view.camera.translation() - position.xyz();

        // Convert color from [0,255] to [0,1]
        let color_vec = glm::vec3(
            pix.channels()[0] as f32 / 255.0,
            pix.channels()[1] as f32 / 255.0,
            pix.channels()[2] as f32 / 255.0,
        );

        // Extract the mask value for this pixel in case we need to update it later
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
        return None;
    } else {
        let checker = brdf::VoxelColoring;

        let result = checker.consistent(&colors_and_rays);

        // Every time a pixel in an image is used to match with a scene element,
        // we need to mask that pixel so it can't be used to match with
        // another scene element
        if let Some(_) = result {
            for mask in masks {
                *mask = true;
            }
        }

        return result;
    }
}

enum XYZ {
    X,
    Y,
    Z,
}

fn plane_sweep(which_plane: XYZ, volume: &mut Volume, views: &mut Vec<View>) -> usize {
    // Our loops bounds depend on which axis the plane we're carving is aligned to
    let loop_bounds = match which_plane {
        XYZ::X => (volume.width, volume.depth, volume.height),
        XYZ::Y => (volume.height, volume.width, volume.depth),
        XYZ::Z => (volume.depth, volume.height, volume.width),
    };
    let mut voxels_carved = 0;
    for a in 0..loop_bounds.0 {
        // Enable this block to write a mesh after each iteration. This is
        // helpful for rendering gifs of the carving process
        if false {
            exporter::write_ply(
                volume,
                &format!("meshes/carve_{:0width$}.ply", a, width = 4),
            );
        }

        // Calculate the plane's position in scene space
        let plane_in_world_space = match which_plane {
            XYZ::X => volume.voxel_to_position(a, 0, 0).x,
            XYZ::Y => volume.voxel_to_position(0, a, 0).y,
            XYZ::Z => volume.voxel_to_position(0, 0, a).z,
        };
        // Find all views which are on one side of the current plane we're carving
        // so that occlusion is consistent.
        let view_is_valid = |t: glm::Vec3| match which_plane {
            XYZ::X => t[0] < plane_in_world_space,
            XYZ::Y => t[1] < plane_in_world_space,
            XYZ::Z => t[2] > plane_in_world_space,
        };
        let mut non_occluded_views: Vec<_> = views
            .iter_mut()
            .filter(|view| view_is_valid(view.camera.translation()))
            .collect();

        println!("Carving plane {} at location {}", a, plane_in_world_space);
        println!("{} views are valid", non_occluded_views.len());

        for b in 0..loop_bounds.1 {
            for c in 0..loop_bounds.2 {
                // Convert loop values into xyz coordinates
                let (x, y, z) = match which_plane {
                    XYZ::X => (a, c, b),
                    XYZ::Y => (b, a, c),
                    XYZ::Z => (c, b, a),
                };

                // if *volume.get_voxel(x, y, z) == Voxel::Carved || !volume.voxel_visible(x, y, z) {
                //     continue;
                // }

                // Perform the voxel carving calculation for this voxel
                let pos_voxel_space = glm::vec3(x as i32, y as i32, z as i32);
                let result = carve_voxel(pos_voxel_space, &volume, &mut non_occluded_views);

                match result {
                    None => {
                        voxels_carved += 1;
                        *volume.get_voxel(x, y, z) = Voxel::Carved;
                    }
                    Some(color) => {
                        *volume.get_voxel(x, y, z) = Voxel::Colored(color);
                    }
                }
            }
        }
    }

    return voxels_carved;
}

/// Given an uncarved volume and a set of views, carve the volume so it is
/// consistent with the views
pub fn carve(volume: &mut Volume, views: &mut Vec<View>) {
    let mut voxels_carved = 0;

    // for alternate algorithms, we could imagine wanting to continue carving
    // until convergence (no more pixels are carved by a single pass). However,
    // for the current algorithm we only need to run once, so this loop has
    // a break at the end.
    loop {
        println!("Carved {} voxels", voxels_carved);

        // Reset per-image masks on each loop iteration
        for view in views.iter_mut() {
            view.reset_mask();
        }

        // Carve the volume starting at the top and sweeping down (works best
        // with cameras positioned above the volume so that occluders
        // are considered first.)
        voxels_carved = plane_sweep(XYZ::Y, volume, views);

        break;
    }

    println!("Carved {} voxels", voxels_carved);
}
