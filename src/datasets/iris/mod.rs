use crate::utils::scale::Scalable;

mod iris;
pub use iris::Iris;

impl Iris {
    pub const NUM_FEATURES: usize = 4;
    pub const FEATURE_NAMES: [&str; Self::NUM_FEATURES] = [
        "sepal length (cm)",
        "sepal width (cm)",
        "petal length (cm)",
        "petal width (cm)",
    ];
    pub const NUM_ROWS: usize = iris::DATA.len();

    pub fn get_as_vec() -> Vec<Self> {
        iris::DATA.to_vec()
    }
}

#[test]
fn count_species() {
    let mut setosa = 0;
    let mut versicolor = 0;
    let mut virginica = 0;

    for i in iris::DATA {
        match i.class {
            "Iris-setosa" => setosa += 1,
            "Iris-versicolor" => versicolor += 1,
            "Iris-virginica" => virginica += 1,
            _ => {}
        }
    }

    assert_eq!(setosa, 50);
    assert_eq!(versicolor, 50);
    assert_eq!(virginica, 50);
}

impl Scalable for Iris {
    const COLUMN_COUNT: usize = todo!();

    fn get_as_f64(&self, index: usize) -> Option<f64> {
        todo!()
    }

    fn set_f64(&mut self, index: usize, value: f64) {
        todo!()
    }
}
