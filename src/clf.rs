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
    let mut features: Vec<usize> = Vec::new();
    let mut best_size = 0;
    let mut best_accuracy = cross_validation(&data, Some(&features));
    println!(
        "{}, Accuracy: {:.1}",
        print_features(&features),
        best_accuracy * 100.0
    );
    for i in 0..data.num_features {
        let mut curr_accuracy = f64::MIN;
        let mut best_feature = 0;
        for j in 0..data.num_features {
            if !features.contains(&j) {
                features.push(j);
                let accuracy = cross_validation(&data, Some(&features));
                println!(
                    "{}, Accuracy: {:.1}",
                    print_features(&features),
                    accuracy * 100.0
                );
                if accuracy > curr_accuracy {
                    best_feature = j;
                    curr_accuracy = accuracy;
                }
                features.pop();
            }
        }
        features.push(best_feature);
        /*
        println!(
            "{}, Accuracy: {:.1}",
            print_features(&features),
            curr_accuracy * 100.0
        );
        */
        if curr_accuracy > best_accuracy {
            best_size = i + 1;
            best_accuracy = curr_accuracy;
        }
    }
    (best_accuracy, features[..best_size].to_vec())
}

pub fn backward_elimination(data: Data) -> (f64, Vec<usize>) {
    let mut features: Vec<usize> = Vec::new();
    let mut best_features: Vec<usize> = Vec::new();
    for i in 0..data.num_features {
        features.push(i);
    }
    let mut best_accuracy = cross_validation(&data, Some(&features));
    println!(
        "With all features, the accuracy is {:.1}%",
        best_accuracy * 100.0
    );
    for _i in 0..data.num_features {
        let mut curr_accuracy = f64::MIN;
        let mut best_feature = 0;
        for j in 0..data.num_features {
            if features.contains(&j) {
                features.swap_remove(features.iter().position(|&x| x == j).unwrap());
                let accuracy = cross_validation(&data, Some(&features));
                println!(
                    "{}, Accuracy: {:.1}%",
                    print_features(&features),
                    accuracy * 100.0
                );
                if accuracy > curr_accuracy {
                    best_feature = j;
                    curr_accuracy = accuracy;
                }
                features.push(j);
            }
        }
        features.swap_remove(features.iter().position(|&x| x == best_feature).unwrap());
        /*
        println!(
            "Feature set {} is best with accuracy {:.1}%",
            print_features(&features),
            curr_accuracy * 100.0
        );
        */
        if curr_accuracy > best_accuracy {
            best_features = features.clone();
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
