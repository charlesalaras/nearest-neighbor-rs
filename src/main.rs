use std::env;

mod clf;
mod data;
mod fread;

use crate::clf::*;
use crate::data::Data;
use crate::fread::read_data;
use std::io;

fn main() {
    /*
    println!("Number of features: {}", data.num_features);
    println!("Total data points: {}", data.set.len());
    for i in 0..data.set.len() {
        println!("Point {}", i);
        println!("class: {}", data.set[i].class.unwrap());
        println!("num features: {}", data.set[i].features.len());
        for feature in &data.set[i].features {
            print!("{} ", feature);
        }
        println!("");
    }
    */
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
    let features = vec![28, 3, 0];
    let accuracy = cross_validation(data, Some(&features));
    println!("Using features 29, 4, 1, the accuracy is: {}", accuracy);
    // All features

    // Forward / Backward Elimination

    // Return the best
}
