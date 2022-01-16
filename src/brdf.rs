use nalgebra_glm as glm;

const THRESHOLD: f32 = 0.06;

pub trait ConsistencyCheck {
    fn consistent(&self, colors_and_rays: &Vec<(glm::Vec3, glm::Vec3)>) -> bool;
}

pub struct VoxelColoring;

impl ConsistencyCheck for VoxelColoring {
    fn consistent(&self, colors_and_rays: &Vec<(glm::Vec3, glm::Vec3)>) -> bool {
        if colors_and_rays.len() == 0 {
            panic!("Can't check consistency of no points");
        }

        // if colors_and_rays
        //     .iter()
        //     .any(|(c, _)| *c == glm::vec3(0.0, 0.0, 0.0))
        // {
        //     return false;
        // }

        let length = colors_and_rays.len();
        let colors = colors_and_rays.iter().map(|(c, _)| c).collect::<Vec<_>>();
        // calculate Σ[X^2]
        let sum_of_colors_squared: glm::Vec3 = colors
            .iter()
            .map(|c| glm::vec3(c.x * c.x, c.y * c.y, c.z * c.z))
            .fold(glm::vec3(0.0, 0.0, 0.0), |acc, c| acc + c);

        // Calculate mu
        let sum_of_colors = colors
            .iter()
            .fold(glm::vec3(0.0, 0.0, 0.0), |acc, c| acc + *c);

        // square sums
        let sum_of_colors = sum_of_colors.component_mul(&sum_of_colors);

        let variance = glm::vec3(
            sum_of_colors_squared.y / length as f32,
            sum_of_colors_squared.x / length as f32,
            sum_of_colors_squared.z / length as f32,
        ) - glm::vec3(
            sum_of_colors.x / (length * length) as f32,
            sum_of_colors.y / (length * length) as f32,
            sum_of_colors.z / (length * length) as f32,
        );

        let threshold_squared = THRESHOLD * THRESHOLD;

        if variance.x < threshold_squared
            && variance.y < threshold_squared
            && variance.z < threshold_squared
        {
            return true;
        } else {
            return false;
        }
    }
}
