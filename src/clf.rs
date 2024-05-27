use crate::data::Data;
use crate::data::Point;
use core::f64::MAX;

pub fn nearest_neighbor(data: Data, point: &mut Point, mask: Option<&[usize]>) -> u32 {
    let mut dist: f64 = MAX;
    // For each data point compare to the given data point at each feature
    for i in data.set {
        let curr_dist = i.dist(point, mask);
        if curr_dist < dist {
            dist = curr_dist;
            point.class = i.class;
        }
    }
    // return a class
    point.class.unwrap()
}

pub fn forward_selection() {
    // Start with no features (default rate)
    let _features: Vec<usize> = Vec::new();
}

pub fn backward_elimination() {
    // Add all features at the start

    // Call K_Fold
}

pub fn k_fold_cross_validation(_k: u32) -> f64 {
    0.0
}
