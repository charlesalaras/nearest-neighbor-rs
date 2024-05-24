pub struct Point {
    class: Option<u32>,
    features: Vec<f64>,
}

impl Point {
    pub fn dist(self: &self, other: &Point) -> f64 {}
}

pub struct Data {
    num_features: u32,
    set: Vec<Point>,
}

impl Data {
    pub fn insert_point(features: Vec<f64>, class: u32) {
        if features.len() == self.num_features {
            set.push_back(Point {
                class: Some(class),
                features,
            });
        }
    }
}
