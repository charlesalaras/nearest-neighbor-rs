use crate::data::Data;
use crate::data::Point;
use core::f64::MAX;

pub fn nearest_neighbor(data: &Data, point: &Point, mask: Option<&[usize]>) -> u32 {
    let mut dist: f64 = MAX;
    let mut class: u32 = 1;
    // For each data point compare to the given data point at each feature
    for i in &data.set {
        if point != i {
            let curr_dist = i.dist(point, mask);
            if curr_dist < dist {
                dist = curr_dist;
                class = i.class.unwrap();
            }
        }
    }
    // return a class
    class
}

pub fn forward_selection() {
    // Start with no features (default rate)
    let _features: Vec<usize> = Vec::new();
    // Attempt to add a feature at each stage

    // Take the best attempt score and append onto that
}

pub fn backward_elimination() {
    // Add all features at the start
    let _features: Vec<usize> = Vec::new();
    // Attempt to remove a feature
}

pub fn cross_validation(data: Data, mask: Option<&[usize]>) -> f64 {
    let mut correct = 0;
    for i in 0..data.set.len() {
        // Slice the data set masking out one k-sized section (offset by i)
        // Call nearest neighbor on the slice
        let prediction = nearest_neighbor(&data, &data.set[i], mask);
        if prediction == data.set[i].class.unwrap() {
            correct += 1;
        }
    }
    correct as f64 / data.set.len() as f64
}
