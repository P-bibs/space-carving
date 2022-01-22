/// This function is for debugging only. It reads in images and backprojects the bounding box points
/// to the image so we can visualize the bounding box.
pub fn visualize_bounding_boxes(volume: &mut Volume, views: &mut Vec<View>) {
    let points_and_colors = vec![
        ((0, 0, 0), [255, 0, 0]),
        ((volume.width, volume.height, volume.depth), [0, 0, 255]),
    ];

    for (i, view) in views.iter_mut().enumerate() {
        let mut copy = view.img.clone().into_rgb8();

        for (point, color) in points_and_colors.iter() {
            let voxel = glm::vec3(point.0 as i32, point.1 as i32, point.2 as i32);

            let color = color.clone();

            let position =
                volume.voxel_to_position(voxel.x as usize, voxel.y as usize, voxel.z as usize);

            let position = glm::vec4(
                position[0] as f32,
                position[1] as f32,
                position[2] as f32,
                1.0,
            );
            let back_projected: glm::Vec3 = view.camera.projection_matrix() * position;

            let back_projected = glm::vec2(
                back_projected.x / back_projected.z,
                back_projected.y / back_projected.z,
            );

            let mut x = back_projected.x.floor() as i32;
            let mut y = back_projected.y.floor() as i32;

            if x < 0 || x >= view.img.width() as i32 || y < 0 || y >= view.img.height() as i32 {
                eprintln!("Back projected point is outside of image bounds");
                continue;
            }

            if x == copy.width() as i32 - 1 {
                x -= 1;
            }
            if y == copy.height() as i32 - 1 {
                y -= 1;
            }

            *copy.get_pixel_mut(x as u32, y as u32) = image::Rgb(color);
            *copy.get_pixel_mut(x as u32 + 1, y as u32 + 1) = image::Rgb(color);
            *copy.get_pixel_mut(x as u32, y as u32 + 1) = image::Rgb(color);
            *copy.get_pixel_mut(x as u32 + 1, y as u32) = image::Rgb(color);
        }
        copy.save(format!("tmp/{:0width$}.png", i, width = 4))
            .unwrap();
    }
}
