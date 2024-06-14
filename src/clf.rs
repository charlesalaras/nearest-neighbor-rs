// Functions for main feature search and nearest neighbor
use crate::data::Data;
use crate::data::Point;
use crate::print_features;
use core::f64::MAX;

/*
Find the nearest neighbor to point, and return the class of that neighbor
@input
- data: The data set
- point: The point to compare against
- mask: List of indices denoting features. Passed to dist to modify Euclidean distance algorithm
@output
- class: Unsigned integer representing the nearest neighbor's class
*/
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
/*
Forward selection which returns the best accuracy and a vector with best feature subset
*/
pub fn forward_selection(data: Data) -> (f64, Vec<usize>) {
    let mut features: Vec<usize> = Vec::new();
    let mut best_size = 0;
    // No features test
    let mut best_accuracy = cross_validation(&data, Some(&features));
    println!(
        "With no features, the accuracy is {:.1}%",
        best_accuracy * 100.0
    );
    // Loop through all features (level search)
    for i in 0..data.num_features {
        let mut curr_accuracy = f64::MIN; // Best accuracy at this level
        let mut best_feature = 0; // Feature we will add
                                  // Test every feature possible
        for j in 0..data.num_features {
            if !features.contains(&j) {
                features.push(j);
                let accuracy = cross_validation(&data, Some(&features));
                println!(
                    "\tUsing features {} with accuracy {:.1}%",
                    print_features(&features),
                    accuracy * 100.0
                );
                if accuracy > curr_accuracy {
                    best_feature = j;
                    curr_accuracy = accuracy;
                }
                // Remove to test a new feature
                features.pop();
            }
        }
        // Insert best feature found
        features.push(best_feature);
        println!(
            "Feature set {} is best with accuracy {:.1}%",
            print_features(&features),
            curr_accuracy * 100.0
        );
        // If better overall, change size to return a slice of said size at end
        if curr_accuracy > best_accuracy {
            best_size = i + 1;
            best_accuracy = curr_accuracy;
        }
    }
    (best_accuracy, features[..best_size].to_vec())
}

/*
Backward elimination which returns the best accuracy and a vector with best feature subset

Note that the usage of swap_remove requires an index,
because of this an iterator is created to find the position of the feature
we want to remove.
*/
pub fn backward_elimination(data: Data) -> (f64, Vec<usize>) {
    let mut features: Vec<usize> = Vec::new();
    let mut best_features: Vec<usize> = Vec::new();
    for i in 0..data.num_features {
        features.push(i);
    }
    // Test with all features
    let mut best_accuracy = cross_validation(&data, Some(&features));
    println!(
        "With all features, the accuracy is {:.1}%",
        best_accuracy * 100.0
    );
    // Loop through all features (level search)
    for _i in 0..data.num_features {
        let mut curr_accuracy = f64::MIN; // Best accuracy at this level
        let mut best_feature = 0; // Feature to remove
                                  // Test removal of each feature (if possible)
        for j in 0..data.num_features {
            if features.contains(&j) {
                // Swap to back and remove
                features.swap_remove(features.iter().position(|&x| x == j).unwrap());
                let accuracy = cross_validation(&data, Some(&features));
                println!(
                    "\tUsing features {} with accuracy {:.1}%",
                    print_features(&features),
                    accuracy * 100.0
                );
                if accuracy > curr_accuracy {
                    best_feature = j;
                    curr_accuracy = accuracy;
                }
                // Add to the back
                features.push(j);
            }
        }
        // Swap best feature to back and remove
        features.swap_remove(features.iter().position(|&x| x == best_feature).unwrap());
        println!(
            "Feature set {} is best with accuracy {:.1}%",
            print_features(&features),
            curr_accuracy * 100.0
        );
        // If best accuracy, clone the current state of the vector
        if curr_accuracy > best_accuracy {
            best_features = features.clone(); // expensive, could cause OOM (?)
            best_accuracy = curr_accuracy;
        }
    }
    (best_accuracy, best_features)
}

/*
Cross validation which calls nearest neighbor, and calculates accuracy as:
correct / number of instances
Note we use number of instances because this is 1-fold cross validation.
*/
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
