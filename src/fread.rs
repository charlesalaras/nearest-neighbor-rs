use crate::data::Data;
use crate::data::Point;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
// https://doc.rust-lang.org/std/io/struct.BufReader.html
pub fn read_data(filename: &str) -> Result<Data, std::io::Error> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);

    let mut points: Vec<Point> = Vec::new();

    for line in reader.lines().flatten() {
        let mut iter = line.split_whitespace();
        let class = iter
            .next()
            .expect("Class not found")
            .parse::<f64>()
            .expect(format!("Given string: {}", line).as_str());
        let mut features: Vec<f64> = Vec::new();
        let mut curr_feature = iter.next();
        while curr_feature.is_some() {
            features.push(
                curr_feature
                    .unwrap()
                    .parse::<f64>()
                    .expect(format!("Given string: {}", line).as_str()),
            );
            curr_feature = iter.next();
        }
        points.push(Point {
            class: Some(class as u32),
            features,
        })
    }

    Ok(Data {
        num_features: points[0].features.len(),
        set: points,
    })
}
