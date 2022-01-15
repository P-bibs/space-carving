use main::Volume;

pub fn write_ply(volume: &mut Volume, filename: &str) {
    let mut out = String::new();

    let mut positions = vec![];
    for z in 0..volume.depth {
        for y in 0..volume.height {
            for x in 0..volume.width {
                if *volume.get_voxel(x, y, z) {
                    let position = volume.voxel_to_position(x, y, z);
                    positions.push(position);
                }
            }
        }
    }

    let mut vertices = vec![];
    let mut faces = vec![];

    for position in &positions {
        let x = position.x;
        let y = position.y;
        let z = position.z;

        let s = volume.voxel_size / 2.0;

        let mut verts = vec![
            (x - s, y - s, z - s),
            (x + s, y - s, z - s),
            (x - s, y + s, z - s),
            (x + s, y + s, z - s),
            (x - s, y - s, z + s),
            (x + s, y - s, z + s),
            (x - s, y + s, z + s),
            (x + s, y + s, z + s),
        ];

        let base = vertices.len();
        let back_bottom_left = base + 0;
        let back_bottom_right = base + 1;
        let back_top_left = base + 2;
        let back_top_right = base + 3;
        let front_bottom_left = base + 4;
        let front_bottom_right = base + 5;
        let front_top_left = base + 6;
        let front_top_right = base + 7;

        // comments are from a looking forward perspective (towards negative z)
        let mut new_faces = vec![
            (
                front_top_left,
                front_top_right,
                back_top_right,
                back_top_left,
            ), // top face
            (
                front_bottom_left,
                front_bottom_right,
                back_bottom_right,
                back_bottom_left,
            ), // bottom face
            (
                front_bottom_right,
                back_bottom_right,
                back_top_right,
                front_top_right,
            ), // right face
            (
                front_bottom_left,
                back_bottom_left,
                back_top_left,
                front_top_left,
            ), // left face
            (
                front_bottom_left,
                front_bottom_right,
                front_top_right,
                front_top_left,
            ), // front face
            (
                back_bottom_left,
                back_bottom_right,
                back_top_right,
                back_top_left,
            ), // back face
        ];

        vertices.append(&mut verts);
        faces.append(&mut new_faces);
    }

    assert_eq!(positions.len() * 8, vertices.len());
    assert_eq!(positions.len() * 6, faces.len());

    out.push_str(&format!(
        r"ply
format ascii 1.0
element vertex {}
property float x
property float y
property float z
element face {}
property list uchar int vertex_indices
end_header
",
        vertices.len(),
        faces.len(),
    ));
    for vertex in vertices {
        out.push_str(&format!("{} {} {}\n", vertex.0, vertex.1, vertex.2));
    }
    for face in faces {
        out.push_str(&format!("4 {} {} {} {}\n", face.0, face.1, face.2, face.3));
    }

    fs::write(filename, out).expect("Unable to write file");
}
