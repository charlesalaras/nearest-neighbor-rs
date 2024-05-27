pub struct Point {
    pub class: Option<u32>,
    pub features: Vec<f64>,
}

impl Point {
    pub fn dist(&self, other: &Point, mask: Option<&[usize]>) -> f64 {
        let mut distance: f64 = 0.0;
        if mask.is_none() {
            for i in 0..self.features.len() {
                distance +=
                    (other.features[i] - self.features[i]) * (other.features[i] - self.features[i]);
            }
        } else {
            for i in mask.unwrap() {
                distance += (other.features[*i] - self.features[*i])
                    * (other.features[*i] - self.features[*i]);
            }
        }
        distance.sqrt()
    }
}

pub struct Data {
    pub num_features: usize,
    pub set: Vec<Point>,
}

impl Data {
    pub fn insert_point(&mut self, features: Vec<f64>, class: u32) {
        if features.len() == self.num_features {
            self.set.push(Point {
                class: Some(class),
                features,
            });
        }
    }
}
