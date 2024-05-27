use std::env;

mod clf;
mod data;
mod fread;

use crate::data::Data;
use crate::fread::read_data;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Please enter a filename");
        return;
    }
    let data: Data = read_data(args[1].clone()).expect("Could not parse file!");

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
}
