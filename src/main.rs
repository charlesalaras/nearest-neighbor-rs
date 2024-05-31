mod clf;
mod data;
mod fread;

use crate::clf::*;
use crate::data::Data;
use crate::fread::read_data;
use std::io;

fn print_features(arr: &[usize]) {
    print!("{{");
    for i in 0..arr.len() - 1 {
        print!("{}, ", arr[i] + 1);
    }
    print!("{}}}", arr[arr.len() - 1] + 1);
}

fn main() {
    println!("Welcome to Charles Alaras' Feature Selection Algorithm");
    println!("Type in the name of the file to test:");
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to parse input");
    let data: Data = read_data(str.trim()).expect("Could not parse file!");
    println!(
        "The dataset has {} features (not including the class attribute), with {} instances",
        data.num_features,
        data.set.len()
    );
    /*
        println!("Type the number of the algorithm you want to run:");
        println!("1) Forward Selection");
        println!("2) Backward Elimination");
        loop {
            io::stdin()
                .read_line(&mut str)
                .expect("Failed to parse input");
        }
    */
    // Test cross validation
    // Note: zero indexed features
    // All features

    // Forward / Backward Elimination
    let (accuracy, best_features) = forward_selection(data);
    //let (accuracy, best_features) = backward_elimination(data);

    print!("\nThe best feature set is: ");
    print_features(&best_features);
    println!(" with accuracy: {}", accuracy);
    // Return the best
}
