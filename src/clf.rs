use crate::data::Data;
use crate::data::Point;
use crate::print_features;
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

pub fn forward_selection(data: Data) -> (f64, Vec<usize>) {
    // Start with no features (default rate)
    let mut best_features: Vec<usize> = Vec::new();
    let mut curr_features: Vec<usize> = Vec::new();
    let mut best_accuracy = cross_validation(&data, Some(&curr_features));
    // Attempt to add a feature at each stage
    for _i in 0..data.num_features {
        let mut curr_accuracy: f64 = f64::MIN;
        let mut best_feature = 0;
        for j in 0..data.num_features {
            if !curr_features.contains(&j) {
                curr_features.push(j);
                print!("Using feature set ");
                print_features(&curr_features);
                print!(" with accuracy: ");
                let accuracy = cross_validation(&data, Some(&curr_features));
                println!("{}", accuracy);
                if accuracy > curr_accuracy {
                    best_feature = j;
                    curr_accuracy = accuracy;
                }
                curr_features.pop();
            }
        }
        curr_features.push(best_feature);
        if curr_accuracy > best_accuracy {
            best_features = curr_features.clone();
            best_accuracy = curr_accuracy;
        }
    }
    (best_accuracy, best_features)
}

pub fn backward_elimination(data: Data) -> (f64, Vec<usize>) {
    // Add all features at the start
    let mut best_features: Vec<usize> = Vec::new();
    let mut curr_features: Vec<usize> = Vec::new();
    for i in 0..data.num_features {
        curr_features.push(i);
    }
    let mut best_accuracy = cross_validation(&data, Some(&curr_features));
    // Attempt to remove a feature
    for _i in 0..data.num_features - 1 {
        let mut curr_accuracy: f64 = f64::MIN;
        let mut best_feature = 0;
        let size = curr_features.len();
        for j in 0..size - 1 {
            curr_features.swap(j, size - 1);
            print!("Using feature set ");
            print_features(&curr_features[..size - 1]);
            print!(" of size {} with accuracy ", size - 1);
            let accuracy = cross_validation(&data, Some(&curr_features[..size - 1]));
            println!("{}", accuracy);
            if accuracy > curr_accuracy {
                best_feature = *curr_features.last().unwrap();
                curr_accuracy = accuracy;
            }
        }
        curr_features.swap_remove(
            curr_features
                .iter()
                .position(|&x| x == best_feature)
                .unwrap(),
        );
        if curr_accuracy > best_accuracy {
            best_features = curr_features.clone();
            best_accuracy = curr_accuracy;
        }
    }
    (best_accuracy, best_features)
}

pub fn cross_validation(data: &Data, mask: Option<&[usize]>) -> f64 {
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
