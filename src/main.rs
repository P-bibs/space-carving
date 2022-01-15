mod brdf;
mod carve;
mod exporter;
mod importer;
mod view;
mod volume;

use crate::volume::Volume;
use nalgebra_glm as glm;

const NUM_IMAGES: usize = 10;

fn main() {
    println!("Loading views");
    let mut views = importer::load_views(NUM_IMAGES);
    println!("Views loaded");

    let voxel_size = 0.01;
    let front_top_left = glm::vec3(-0.023121, 0.121636, -0.017395);
    let back_bottom_right = glm::vec3(0.078626, -0.038009, -0.091940);
    let mut volume = Volume::new(voxel_size, front_top_left, back_bottom_right);

    let mut voxels_carved = 0;
    loop {
        println!("Carved {} voxels", voxels_carved);
        let mut converged = true;

        for view in &mut views {
            view.reset_mask();
        }

        for plane_index in 0..volume.depth {
            let plane_in_world_space = volume.voxel_to_position(0, 0, plane_index).z;
            println!(
                "Carving plane {} at location {}",
                plane_index, plane_in_world_space
            );
            let mut non_occluded_views: Vec<_> = views
                .iter_mut()
                .filter(|view| view.camera.translation()[2] > plane_in_world_space)
                .collect();

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

                    let result =
                        carve::carve_voxel(pos_voxel_space, &volume, &mut non_occluded_views);

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

    exporter::write_ply(&mut volume, "out.ply");

    println!("Finished. Carved {} voxels", voxels_carved);
}
