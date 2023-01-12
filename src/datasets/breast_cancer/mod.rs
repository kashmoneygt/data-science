mod breast_cancer;
pub use breast_cancer::BreastCancer;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Target {
    Malignant,
    Benign,
}

impl Target {
    pub const fn to_str(&self) -> &str {
        match self {
            Target::Malignant => "Malignant",
            Target::Benign => "Benign",
        }
    }
}

impl BreastCancer {
    pub const NUM_FEATURES: usize = 30;
    pub const FEATURE_NAMES: [&str; Self::NUM_FEATURES] = [
        "mean radius",
        "mean texture",
        "mean perimeter",
        "mean area",
        "mean smoothness",
        "mean compactness",
        "mean concavity",
        "mean concave points",
        "mean symmetry",
        "mean fractal dimension",
        "radius error",
        "texture error",
        "perimeter error",
        "area error",
        "smoothness error",
        "compactness error",
        "concavity error",
        "concave points error",
        "symmetry error",
        "fractal dimension error",
        "worst radius",
        "worst texture",
        "worst perimeter",
        "worst area",
        "worst smoothness",
        "worst compactness",
        "worst concavity",
        "worst concave points",
        "worst symmetry",
        "worst fractal dimension",
    ];
    pub const TARGET_NAMES: [&str; 2] = [Target::Malignant.to_str(), Target::Benign.to_str()];
    pub const NUM_ROWS: usize = breast_cancer::DATA.len();

    pub fn get_as_vec() -> Vec<Self> {
        breast_cancer::DATA.to_vec()
    }
}

#[test]
fn count_target_classes() {
    let mut malignant = 0;
    let mut benign = 0;

    for i in breast_cancer::DATA {
        match i.target {
            0 => malignant += 1,
            1 => benign += 1,
            _ => {}
        }
    }

    assert_eq!(benign, 357);
    assert_eq!(malignant, 212);
}
