use crate::view::{CameraData, View};
use indicatif::ProgressIterator;
use std::fs;

pub fn load_views(location: &str, prefix: &str, num_images: usize) -> Vec<View> {
    assert!(location.ends_with("/"));
    let metadata_filename = format!("{}{}_par.txt", location, prefix);

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

    let images = (1..num_images)
        .map(|i| format!("{}{}{:0width$}.png", location, prefix, i, width = 4))
        .map(|filename| image::open(filename).expect("Couldn't open file"))
        .progress();

    let views: Vec<View> = metadata
        .zip(images)
        .map(|(camera, img)| View::new(camera, img))
        .collect();

    return views;
}
