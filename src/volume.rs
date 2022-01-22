use nalgebra_glm as glm;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }
    pub fn from_vec3(v: glm::Vec3) -> Color {
        Color {
            r: v.x,
            g: v.y,
            b: v.z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Voxel {
    Carved,
    Untouched,
    Colored(Color),
}

/// A struct to represent a 3d volume of voxels.
pub struct Volume {
    pub data: Vec<Vec<Vec<Voxel>>>,
    pub voxel_size: f32,
    pub front_top_left: glm::Vec3,
    pub back_bottom_right: glm::Vec3,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}
impl Volume {
    /// create a new volume with bounding box defined by front_top_left and back_bottom_right, with
    /// voxels of size voxel_size.
    pub fn new(voxel_size: f32, front_top_left: glm::Vec3, back_bottom_right: glm::Vec3) -> Self {
        // Determine dimensions in # of voxels
        let width = ((back_bottom_right.x - front_top_left.x).abs() / voxel_size).ceil() as usize;
        let height = ((back_bottom_right.y - front_top_left.y).abs() / voxel_size).ceil() as usize;
        let depth = ((back_bottom_right.z - front_top_left.z).abs() / voxel_size).ceil() as usize;

        // Ensure coordinates are even so the origin doesn't fall between voxels
        let width = if width % 2 == 1 { width + 1 } else { width };
        let height = if height % 2 == 1 { height + 1 } else { height };
        let depth = if depth % 2 == 1 { depth + 1 } else { depth };

        // Initialize a 3d vector of voxels
        let mut cols = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                let mut depth_line = vec![];
                for _ in 0..depth {
                    depth_line.push(Voxel::Untouched);
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
        let y = self.front_top_left.y - (y as f32 * self.voxel_size) - (self.voxel_size / 2.0);
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
            if self.data[y][x][z] == Voxel::Carved {
                return true;
            }
        }

        return false;
    }
    /// Get a mutable reference to a voxel at the given indices
    pub fn get_voxel(&mut self, x: usize, y: usize, z: usize) -> &mut Voxel {
        &mut self.data[y][x][z]
    }
}
