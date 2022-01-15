use nalgebra_glm as glm;
use std::cmp;

const THRESHOLD: f32 = 0.1;

pub trait ConsistencyCheck {
    fn consistent(&self, colors_and_rays: &Vec<(glm::Vec3, glm::Vec3)>) -> bool;
}

pub struct VoxelColoring;

impl ConsistencyCheck for VoxelColoring {
    fn consistent(&self, colors_and_rays: &Vec<(glm::Vec3, glm::Vec3)>) -> bool {
        if colors_and_rays.len() == 0 {
            panic!("Can't check consistency of no points");
        }

        let length = colors_and_rays.len();
        let colors = colors_and_rays.iter().map(|(c, _)| c).collect::<Vec<_>>();
        // calculate Σ[X^2]
        let sum_of_colors_squared: glm::Vec3 = colors
            .iter()
            .map(|c| glm::vec3(c.x * c.x, c.y * c.y, c.z * c.z))
            .fold(glm::vec3(0.0, 0.0, 0.0), |acc, c| acc + c);

        // Calculate mu
        let average_color = colors
            .iter()
            .fold(glm::vec3(0.0, 0.0, 0.0), |acc, c| acc + *c)
            / (length as f32);

        // variance is Σ[X^2] / N - mu^2
        let variance =
            (sum_of_colors_squared / length as f32) - average_color.component_mul(&average_color);

        let mut max_variance = variance.x;
        if variance.y > max_variance {
            max_variance = variance.y;
        }
        if variance.z > max_variance {
            max_variance = variance.z;
        }

        let standard_deviation = max_variance.sqrt();

        // println!("Standard deviation of {}", standard_deviation);

        return standard_deviation < THRESHOLD;
    }
}
