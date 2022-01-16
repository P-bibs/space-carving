use nalgebra_glm as glm;

pub struct Volume {
    pub data: Vec<Vec<Vec<bool>>>,
    pub voxel_size: f32,
    pub front_top_left: glm::Vec3,
    pub back_bottom_right: glm::Vec3,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}
impl Volume {
    pub fn new(voxel_size: f32, front_top_left: glm::Vec3, back_bottom_right: glm::Vec3) -> Self {
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
                    depth_line.push(true);
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
            front_top_left,
            back_bottom_right,
            width: width as usize,
            height: height as usize,
            depth: depth as usize,
        }
    }
    pub fn voxel_to_position(&self, x: usize, y: usize, z: usize) -> glm::Vec3 {
        let x = self.front_top_left.x + (x as f32 * self.voxel_size) + (self.voxel_size / 2.0);
        let y = self.front_top_left.y + (y as f32 * self.voxel_size) + (self.voxel_size / 2.0);
        let z = self.front_top_left.z - (z as f32 * self.voxel_size) - (self.voxel_size / 2.0);

        return glm::vec3(x, y, z);
    }

    /// true if any of the six voxels surrounding this voxel are missing, false otherwise
    pub fn voxel_visible(&self, x: usize, y: usize, z: usize) -> bool {
        if x >= self.width || y >= self.height || z >= self.depth {
            panic!("Voxel out of bounds");
        }

        // If the voxel is on the edge of the volume, it's visible
        if x == 0
            || y == 0
            || z == 0
            || x == self.width - 1
            || y == self.height - 1
            || z == self.depth - 1
        {
            return true;
        }

        // enumerate all neighboring voxel coords and check if they are present
        let coords = vec![
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ];

        for (x, y, z) in coords {
            // If a neighboring voxel isn't present, then this voxel is visible
            if self.data[y][x][z] == false {
                return true;
            }
        }

        return false;
    }
    pub fn get_voxel(&mut self, x: usize, y: usize, z: usize) -> &mut bool {
        &mut self.data[y][x][z]
    }
    pub fn get_voxel_ws(&mut self, x: f32, y: f32, z: f32) -> &mut bool {
        print!("Converting index at ({}, {}, {}) to voxel index: ", x, y, z);

        // Flip y and z to match the 3d array coordinate system (origin in front-top-left)
        let x = x;
        let y = -y;
        let z = -z;

        // Since the voxels are not unit size, we need to scale the coordinates
        // to the correct voxel.
        let x = x / self.voxel_size;
        let y = y / self.voxel_size;
        let z = z / self.voxel_size;

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
