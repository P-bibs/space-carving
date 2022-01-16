mod brdf;
mod carve;
mod exporter;
mod importer;
mod view;
mod volume;

use crate::volume::Volume;
use nalgebra_glm as glm;

const NUM_IMAGES: usize = 46;

fn main() {
    println!("Loading views");
    let mut views = importer::load_views(NUM_IMAGES);
    println!("Views loaded");

    let voxel_size = 0.001;
    let front_top_left = glm::vec3(-0.023121, -0.038009, -0.017395);
    let back_bottom_right = glm::vec3(0.078626, 0.121636, -0.091940);

    // let front_top_left = glm::vec3(-0.023121, -0.017395, 0.121636);
    // let back_bottom_right = glm::vec3(0.078626, -0.091940, -0.038009);

    let mut volume = Volume::new(voxel_size, front_top_left, back_bottom_right);

    carve::carve(&mut volume, &mut views);

    exporter::write_ply(&mut volume, "out.ply");
}
