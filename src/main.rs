mod clf;
mod data;
mod fread;

use crate::clf::*;
use crate::data::Data;
use crate::fread::read_data;
use std::io;
use std::time::Instant;

// Utility function to print feature subsets as "{1, 2, 3}"
// Requires a slice of a vector
fn print_features(arr: &[usize]) -> String {
    if arr.len() == 0 {
        return String::from("{}");
    }
    let mut s = String::from("{");
    for i in 0..arr.len() - 1 {
        s.push_str(format!("{}, ", arr[i] + 1).as_str());
    }
    s.push_str(format!("{}}}", arr[arr.len() - 1] + 1).as_str());
    s
}

fn main() {
    println!("Welcome to Charles Alaras' Feature Selection Algorithm");
    println!("Type in the name of the file to test:");
    // Read user input and try to use it as a filename
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to parse input");
    // NOTE: A panic will occur here if something goes wrong with the file
    let data: Data = read_data(str.trim()).expect("Could not parse file!");
    println!(
        "The dataset has {} features (not including the class attribute), with {} instances",
        data.num_features,
        data.set.len()
    );
    // Read user input and parse as a number (1 or 2)
    println!("Type the number of the algorithm you want to run:");
    println!("1) Forward Selection");
    println!("2) Backward Elimination");
    let mut result;
    loop {
        str.clear();
        io::stdin()
            .read_line(&mut str)
            .expect("Failed to parse input");
        let number = str.trim().parse::<u32>();
        if number.is_err() {
            println!("ERROR!: Received string: {}", str);
        } else {
            result = number.unwrap();
            if result != 1 && result != 2 {
                println!("ERROR!: Please enter 1 or 2.");
            } else {
                break;
            }
        }
    }
    // Start timer
    let start_time: Instant = Instant::now();
    // Run forward_selection / backward_elimination based on above result
    let (accuracy, best_features) = if result == 1 {
        forward_selection(data)
    } else {
        backward_elimination(data)
    };

    let duration = start_time.elapsed(); //ends timer

    println!(
        "\nThe best feature set is: {} with accuracy {:.1}%",
        print_features(&best_features),
        accuracy * 100.0
    );

    println!("Runtime: {:.2}s", duration.as_secs_f32());
}
