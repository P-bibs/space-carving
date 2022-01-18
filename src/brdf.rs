use crate::volume::Color;
use nalgebra_glm as glm;

const THRESHOLD: f32 = 0.3;

pub trait ConsistencyCheck {
    fn consistent(&self, colors_and_rays: &Vec<(glm::Vec3, glm::Vec3)>) -> Option<Color>;
}

pub struct VoxelColoring;

impl ConsistencyCheck for VoxelColoring {
    fn consistent(&self, colors_and_rays: &Vec<(glm::Vec3, glm::Vec3)>) -> Option<Color> {
        if colors_and_rays.len() == 0 {
            panic!("Can't check consistency of no points");
        }

        if colors_and_rays
            .iter()
            .any(|(c, _)| *c == glm::vec3(0.0, 0.0, 0.0))
        {
            return None;
        }

        let length = colors_and_rays.len();
        let colors = colors_and_rays.iter().map(|(c, _)| c).collect::<Vec<_>>();

        let sum_of_colors_squared: glm::Vec3 = colors
            .iter()
            .map(|c| glm::vec3(c.x * c.x, c.y * c.y, c.z * c.z))
            .fold(glm::vec3(0.0, 0.0, 0.0), |acc, c| acc + c);

        let sum_of_colors = colors
            .iter()
            .fold(glm::vec3(0.0, 0.0, 0.0), |acc, c| acc + *c);

        let average_color = sum_of_colors / (length as f32);

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

        if average_color.x < 0.2 && average_color.y < 0.2 && average_color.z < 0.2 {
            return None;
        }

        let threshold_squared = THRESHOLD * THRESHOLD;

        if variance.x < threshold_squared
            && variance.y < threshold_squared
            && variance.z < threshold_squared
        {
            // Don't carve pixel
            return Some(Color::from_vec3(average_color));
        } else {
            // Carve pixel
            return None;
        }
    }
}
