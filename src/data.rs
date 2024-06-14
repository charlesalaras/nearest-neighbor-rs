/*
Represents a data point
Note the Option<u32> for class. This allows points to not have a class if necessary,
but it will cause issues if no class exists.
*/
pub struct Point {
    pub class: Option<u32>,
    pub features: Vec<f64>,
}

impl Point {
    // Calculates Euclidean distance of a point to self
    // Note that the dimensions is controlled by the mask
    // If no mask, just loop through all the features
    pub fn dist(&self, other: &Point, mask: Option<&[usize]>) -> f64 {
        let mut distance: f64 = 0.0;
        if mask.is_none() {
            for i in 0..self.features.len() {
                distance +=
                    (other.features[i] - self.features[i]) * (other.features[i] - self.features[i]);
            }
        } else {
            // Have to dereference i here as we're unwrapping a mask to get a reference slice
            for i in mask.unwrap() {
                distance += (other.features[*i] - self.features[*i])
                    * (other.features[*i] - self.features[*i]);
            }
        }
        distance.sqrt()
    }
}

impl PartialEq for Point {
    // Implements point equality
    fn eq(&self, other: &Self) -> bool {
        for i in 0..self.features.len() {
            if self.features[i] != other.features[i] {
                return false;
            }
        }
        // Disregard class, if features are same then this is the same point
        true
    }
}
impl Eq for Point {}

pub struct Data {
    pub num_features: usize,
    pub set: Vec<Point>,
}
