mod brdf;
mod carve;
mod exporter;
mod importer;
mod view;
mod volume;

use crate::volume::Volume;
use clap::Parser;
use nalgebra_glm as glm;
use serde::{Deserialize, Serialize};
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Folder with a dataset
    #[clap(short, long)]
    dataset: String,

    /// number of images to load
    #[clap(short, long)]
    num_images: usize,

    /// File to write .ply to
    #[clap(short, long, default_value = "carved.ply")]
    output: String,

    /// The size of a voxel
    #[clap(short, long, default_value_t = 0.001)]
    voxel_size: f32,

    /// The threshold of the carving algorithm
    /// The lower the value, the more pixels will be carved
    #[clap(short, long, default_value_t = 0.3)]
    threshold: f32,
}

#[derive(Serialize, Deserialize)]
struct Config {
    directory: String,
    prefix: String,
    // Bounding box coords
    bb_front_top_left: [f32; 3],
    bb_back_bottom_right: [f32; 3],
}

fn main() {
    let args = Args::parse();

    let dataset = fs::read_to_string(args.dataset).expect("Couldn't read dataset file");

    // deserialize file to a config struct
    let config: Config = serde_json::from_str(&dataset).unwrap();

    println!("Loading views");
    let mut views = importer::load_views(&config.directory, &config.prefix, args.num_images);
    println!("Views loaded");

    let bb_front_top_left = glm::vec3(
        config.bb_front_top_left[0],
        config.bb_front_top_left[1],
        config.bb_front_top_left[2],
    );
    let bb_back_bottom_right = glm::vec3(
        config.bb_back_bottom_right[0],
        config.bb_back_bottom_right[1],
        config.bb_back_bottom_right[2],
    );

    let mut volume = Volume::new(args.voxel_size, bb_front_top_left, bb_back_bottom_right);

    // perform the carving
    carve::carve(&mut volume, &mut views, args.threshold);

    // Output the result
    exporter::write_ply(&mut volume, &args.output);
}
