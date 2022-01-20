/// this file contains different methods for performing a consistency check for
/// different views of the same location in a scene to see if the pixel colors
/// reported by the different views are consistent and therefore if that
/// location is actually part of the scene volume.
///
/// The `ConsistencyCheck` trait defines the interface for this check, and
/// different implementors of that trait define different consistency checking
/// methodologies.
use crate::volume::Color;
use nalgebra_glm as glm;

const THRESHOLD: f32 = 0.5;

/// perform consistency checking via the voxel coloring algorithm. This assumes
/// a lambertian radiance function which means that the color of a scene element
/// should be view-independent. A set of views are deemed to be consistent
/// if the standard deviation of their perceived colors is below a certain threshold
pub fn standard_consistency_check(colors: &Vec<glm::Vec3>, threshold: f32) -> Option<Color> {
    if colors.len() == 0 {
        panic!("Can't check consistency of no points");
    }

    // Assuming a black background, if any camera sees a background pixel then
    // this scene element cannot possibly exist
    if colors
        .iter()
        .any(|c| *c == glm::vec3(0.0, 0.0, 0.0))
    {
        return None;
    }

    // calculate number of views and extract just the color values for each view
    let length = colors.len();

    // Calculate variance:
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

    // Similar to above, near-black average color values indicates that every
    // view is seeing a black pixel, which means they are seeing background
    // and this element should be carved.
    if average_color.x < 0.2 && average_color.y < 0.2 && average_color.z < 0.2 {
        return None;
    }

    let threshold_squared = threshold * threshold;

    // ensure each channel is below the threshold
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
