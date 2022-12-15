/// [Iris plants dataset](https://github.com/scikit-learn/scikit-learn/blob/dc580a8ef5ee2a8aea80498388690e2213118efd/sklearn/datasets/descr/iris.rst) with 4 numeric, predictive attributes and the class.
///
/// The data set contains 3 classes of 50 instances each, where each class refers to a type of iris plant. One class is linearly separable from the other 2; the latter are NOT linearly separable from each other.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Iris {
    /// Length of the sepal in centimeters
    pub sepal_length: f32,

    /// Width of the sepal in centimeters
    pub sepal_width: f32,

    /// Length of the petal in centimeters
    pub petal_length: f32,

    /// Width of the petal in centimeters
    pub petal_width: f32,

    /// Identified [Species] of iris
    pub species: Species,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Species {
    /// [Iris setosa](https://en.wikipedia.org/wiki/Iris_setosa), the bristle-pointed iris.
    IrisSetosa,

    /// [Iris versicolor](https://en.wikipedia.org/wiki/Iris_versicolor), commonly known as the
    /// blue flag.
    IrisVersicolor,

    /// [Iris virginica](https://en.wikipedia.org/wiki/Iris_virginica), commonly known as the
    /// Virginia blueflag.
    IrisVirginica,
}

impl Species {
    pub const fn to_str(&self) -> &str {
        match self {
            Self::IrisSetosa => "Iris setosa",
            Self::IrisVersicolor => "Iris versicolor",
            Self::IrisVirginica => "Iris virginica",
        }
    }
}

impl Iris {
    pub const NUM_FEATURES: usize = 4;
    pub const FEATURE_NAMES: [&str; Self::NUM_FEATURES] = [
        "sepal length (cm)",
        "sepal width (cm)",
        "petal length (cm)",
        "petal width (cm)",
    ];
    pub const TARGET_NAMES: [&str; 3] = [
        Species::IrisSetosa.to_str(),
        Species::IrisVersicolor.to_str(),
        Species::IrisVirginica.to_str(),
    ];
}

macro_rules! iris_row {
    ($sepal_length: literal, $sepal_width: literal, $petal_length: literal, $petal_width: literal, $species: ident) => {
        Iris {
            sepal_length: $sepal_length,
            sepal_width: $sepal_width,
            petal_length: $petal_length,
            petal_width: $petal_width,
            species: Species::$species,
        }
    };
}

/// [Fisher's Iris data set](https://en.wikipedia.org/wiki/Iris_flower_data_set).
pub const DATA: &[Iris; 150] = &[
    iris_row!(5.1, 3.5, 1.4, 0.2, IrisSetosa),
    iris_row!(4.9, 3.0, 1.4, 0.2, IrisSetosa),
    iris_row!(4.7, 3.2, 1.3, 0.2, IrisSetosa),
    iris_row!(4.6, 3.1, 1.5, 0.2, IrisSetosa),
    iris_row!(5.0, 3.6, 1.4, 0.3, IrisSetosa),
    iris_row!(5.4, 3.9, 1.7, 0.4, IrisSetosa),
    iris_row!(4.6, 3.4, 1.4, 0.3, IrisSetosa),
    iris_row!(5.0, 3.4, 1.5, 0.2, IrisSetosa),
    iris_row!(4.4, 2.9, 1.4, 0.2, IrisSetosa),
    iris_row!(4.9, 3.1, 1.5, 0.1, IrisSetosa),
    iris_row!(5.4, 3.7, 1.5, 0.2, IrisSetosa),
    iris_row!(4.8, 3.4, 1.6, 0.2, IrisSetosa),
    iris_row!(4.8, 3.0, 1.4, 0.1, IrisSetosa),
    iris_row!(4.3, 3.0, 1.1, 0.1, IrisSetosa),
    iris_row!(5.8, 4.0, 1.2, 0.2, IrisSetosa),
    iris_row!(5.7, 4.4, 1.5, 0.4, IrisSetosa),
    iris_row!(5.4, 3.9, 1.3, 0.4, IrisSetosa),
    iris_row!(5.1, 3.5, 1.4, 0.3, IrisSetosa),
    iris_row!(5.7, 3.8, 1.7, 0.3, IrisSetosa),
    iris_row!(5.1, 3.8, 1.5, 0.3, IrisSetosa),
    iris_row!(5.4, 3.4, 1.7, 0.2, IrisSetosa),
    iris_row!(5.1, 3.7, 1.5, 0.4, IrisSetosa),
    iris_row!(4.6, 3.6, 1.0, 0.2, IrisSetosa),
    iris_row!(5.1, 3.3, 1.7, 0.5, IrisSetosa),
    iris_row!(4.8, 3.4, 1.9, 0.2, IrisSetosa),
    iris_row!(5.0, 3.0, 1.6, 0.2, IrisSetosa),
    iris_row!(5.0, 3.4, 1.6, 0.4, IrisSetosa),
    iris_row!(5.2, 3.5, 1.5, 0.2, IrisSetosa),
    iris_row!(5.2, 3.4, 1.4, 0.2, IrisSetosa),
    iris_row!(4.7, 3.2, 1.6, 0.2, IrisSetosa),
    iris_row!(4.8, 3.1, 1.6, 0.2, IrisSetosa),
    iris_row!(5.4, 3.4, 1.5, 0.4, IrisSetosa),
    iris_row!(5.2, 4.1, 1.5, 0.1, IrisSetosa),
    iris_row!(5.5, 4.2, 1.4, 0.2, IrisSetosa),
    iris_row!(4.9, 3.1, 1.5, 0.2, IrisSetosa),
    iris_row!(5.0, 3.2, 1.2, 0.2, IrisSetosa),
    iris_row!(5.5, 3.5, 1.3, 0.2, IrisSetosa),
    iris_row!(4.9, 3.6, 1.4, 0.1, IrisSetosa),
    iris_row!(4.4, 3.0, 1.3, 0.2, IrisSetosa),
    iris_row!(5.1, 3.4, 1.5, 0.2, IrisSetosa),
    iris_row!(5.0, 3.5, 1.3, 0.3, IrisSetosa),
    iris_row!(4.5, 2.3, 1.3, 0.3, IrisSetosa),
    iris_row!(4.4, 3.2, 1.3, 0.2, IrisSetosa),
    iris_row!(5.0, 3.5, 1.6, 0.6, IrisSetosa),
    iris_row!(5.1, 3.8, 1.9, 0.4, IrisSetosa),
    iris_row!(4.8, 3.0, 1.4, 0.3, IrisSetosa),
    iris_row!(5.1, 3.8, 1.6, 0.2, IrisSetosa),
    iris_row!(4.6, 3.2, 1.4, 0.2, IrisSetosa),
    iris_row!(5.3, 3.7, 1.5, 0.2, IrisSetosa),
    iris_row!(5.0, 3.3, 1.4, 0.2, IrisSetosa),
    iris_row!(7.0, 3.2, 4.7, 1.4, IrisVersicolor),
    iris_row!(6.4, 3.2, 4.5, 1.5, IrisVersicolor),
    iris_row!(6.9, 3.1, 4.9, 1.5, IrisVersicolor),
    iris_row!(5.5, 2.3, 4.0, 1.3, IrisVersicolor),
    iris_row!(6.5, 2.8, 4.6, 1.5, IrisVersicolor),
    iris_row!(5.7, 2.8, 4.5, 1.3, IrisVersicolor),
    iris_row!(6.3, 3.3, 4.7, 1.6, IrisVersicolor),
    iris_row!(4.9, 2.4, 3.3, 1.0, IrisVersicolor),
    iris_row!(6.6, 2.9, 4.6, 1.3, IrisVersicolor),
    iris_row!(5.2, 2.7, 3.9, 1.4, IrisVersicolor),
    iris_row!(5.0, 2.0, 3.5, 1.0, IrisVersicolor),
    iris_row!(5.9, 3.0, 4.2, 1.5, IrisVersicolor),
    iris_row!(6.0, 2.2, 4.0, 1.0, IrisVersicolor),
    iris_row!(6.1, 2.9, 4.7, 1.4, IrisVersicolor),
    iris_row!(5.6, 2.9, 3.6, 1.3, IrisVersicolor),
    iris_row!(6.7, 3.1, 4.4, 1.4, IrisVersicolor),
    iris_row!(5.6, 3.0, 4.5, 1.5, IrisVersicolor),
    iris_row!(5.8, 2.7, 4.1, 1.0, IrisVersicolor),
    iris_row!(6.2, 2.2, 4.5, 1.5, IrisVersicolor),
    iris_row!(5.6, 2.5, 3.9, 1.1, IrisVersicolor),
    iris_row!(5.9, 3.2, 4.8, 1.8, IrisVersicolor),
    iris_row!(6.1, 2.8, 4.0, 1.3, IrisVersicolor),
    iris_row!(6.3, 2.5, 4.9, 1.5, IrisVersicolor),
    iris_row!(6.1, 2.8, 4.7, 1.2, IrisVersicolor),
    iris_row!(6.4, 2.9, 4.3, 1.3, IrisVersicolor),
    iris_row!(6.6, 3.0, 4.4, 1.4, IrisVersicolor),
    iris_row!(6.8, 2.8, 4.8, 1.4, IrisVersicolor),
    iris_row!(6.7, 3.0, 5.0, 1.7, IrisVersicolor),
    iris_row!(6.0, 2.9, 4.5, 1.5, IrisVersicolor),
    iris_row!(5.7, 2.6, 3.5, 1.0, IrisVersicolor),
    iris_row!(5.5, 2.4, 3.8, 1.1, IrisVersicolor),
    iris_row!(5.5, 2.4, 3.7, 1.0, IrisVersicolor),
    iris_row!(5.8, 2.7, 3.9, 1.2, IrisVersicolor),
    iris_row!(6.0, 2.7, 5.1, 1.6, IrisVersicolor),
    iris_row!(5.4, 3.0, 4.5, 1.5, IrisVersicolor),
    iris_row!(6.0, 3.4, 4.5, 1.6, IrisVersicolor),
    iris_row!(6.7, 3.1, 4.7, 1.5, IrisVersicolor),
    iris_row!(6.3, 2.3, 4.4, 1.3, IrisVersicolor),
    iris_row!(5.6, 3.0, 4.1, 1.3, IrisVersicolor),
    iris_row!(5.5, 2.5, 4.0, 1.3, IrisVersicolor),
    iris_row!(5.5, 2.6, 4.4, 1.2, IrisVersicolor),
    iris_row!(6.1, 3.0, 4.6, 1.4, IrisVersicolor),
    iris_row!(5.8, 2.6, 4.0, 1.2, IrisVersicolor),
    iris_row!(5.0, 2.3, 3.3, 1.0, IrisVersicolor),
    iris_row!(5.6, 2.7, 4.2, 1.3, IrisVersicolor),
    iris_row!(5.7, 3.0, 4.2, 1.2, IrisVersicolor),
    iris_row!(5.7, 2.9, 4.2, 1.3, IrisVersicolor),
    iris_row!(6.2, 2.9, 4.3, 1.3, IrisVersicolor),
    iris_row!(5.1, 2.5, 3.0, 1.1, IrisVersicolor),
    iris_row!(5.7, 2.8, 4.1, 1.3, IrisVersicolor),
    iris_row!(6.3, 3.3, 6.0, 2.5, IrisVirginica),
    iris_row!(5.8, 2.7, 5.1, 1.9, IrisVirginica),
    iris_row!(7.1, 3.0, 5.9, 2.1, IrisVirginica),
    iris_row!(6.3, 2.9, 5.6, 1.8, IrisVirginica),
    iris_row!(6.5, 3.0, 5.8, 2.2, IrisVirginica),
    iris_row!(7.6, 3.0, 6.6, 2.1, IrisVirginica),
    iris_row!(4.9, 2.5, 4.5, 1.7, IrisVirginica),
    iris_row!(7.3, 2.9, 6.3, 1.8, IrisVirginica),
    iris_row!(6.7, 2.5, 5.8, 1.8, IrisVirginica),
    iris_row!(7.2, 3.6, 6.1, 2.5, IrisVirginica),
    iris_row!(6.5, 3.2, 5.1, 2.0, IrisVirginica),
    iris_row!(6.4, 2.7, 5.3, 1.9, IrisVirginica),
    iris_row!(6.8, 3.0, 5.5, 2.1, IrisVirginica),
    iris_row!(5.7, 2.5, 5.0, 2.0, IrisVirginica),
    iris_row!(5.8, 2.8, 5.1, 2.4, IrisVirginica),
    iris_row!(6.4, 3.2, 5.3, 2.3, IrisVirginica),
    iris_row!(6.5, 3.0, 5.5, 1.8, IrisVirginica),
    iris_row!(7.7, 3.8, 6.7, 2.2, IrisVirginica),
    iris_row!(7.7, 2.6, 6.9, 2.3, IrisVirginica),
    iris_row!(6.0, 2.2, 5.0, 1.5, IrisVirginica),
    iris_row!(6.9, 3.2, 5.7, 2.3, IrisVirginica),
    iris_row!(5.6, 2.8, 4.9, 2.0, IrisVirginica),
    iris_row!(7.7, 2.8, 6.7, 2.0, IrisVirginica),
    iris_row!(6.3, 2.7, 4.9, 1.8, IrisVirginica),
    iris_row!(6.7, 3.3, 5.7, 2.1, IrisVirginica),
    iris_row!(7.2, 3.2, 6.0, 1.8, IrisVirginica),
    iris_row!(6.2, 2.8, 4.8, 1.8, IrisVirginica),
    iris_row!(6.1, 3.0, 4.9, 1.8, IrisVirginica),
    iris_row!(6.4, 2.8, 5.6, 2.1, IrisVirginica),
    iris_row!(7.2, 3.0, 5.8, 1.6, IrisVirginica),
    iris_row!(7.4, 2.8, 6.1, 1.9, IrisVirginica),
    iris_row!(7.9, 3.8, 6.4, 2.0, IrisVirginica),
    iris_row!(6.4, 2.8, 5.6, 2.2, IrisVirginica),
    iris_row!(6.3, 2.8, 5.1, 1.5, IrisVirginica),
    iris_row!(6.1, 2.6, 5.6, 1.4, IrisVirginica),
    iris_row!(7.7, 3.0, 6.1, 2.3, IrisVirginica),
    iris_row!(6.3, 3.4, 5.6, 2.4, IrisVirginica),
    iris_row!(6.4, 3.1, 5.5, 1.8, IrisVirginica),
    iris_row!(6.0, 3.0, 4.8, 1.8, IrisVirginica),
    iris_row!(6.9, 3.1, 5.4, 2.1, IrisVirginica),
    iris_row!(6.7, 3.1, 5.6, 2.4, IrisVirginica),
    iris_row!(6.9, 3.1, 5.1, 2.3, IrisVirginica),
    iris_row!(5.8, 2.7, 5.1, 1.9, IrisVirginica),
    iris_row!(6.8, 3.2, 5.9, 2.3, IrisVirginica),
    iris_row!(6.7, 3.3, 5.7, 2.5, IrisVirginica),
    iris_row!(6.7, 3.0, 5.2, 2.3, IrisVirginica),
    iris_row!(6.3, 2.5, 5.0, 1.9, IrisVirginica),
    iris_row!(6.5, 3.0, 5.2, 2.0, IrisVirginica),
    iris_row!(6.2, 3.4, 5.4, 2.3, IrisVirginica),
    iris_row!(5.9, 3.0, 5.1, 1.8, IrisVirginica),
];

#[test]
fn count_species() {
    let mut setosa = 0;
    let mut versicolor = 0;
    let mut virginica = 0;

    for i in DATA {
        match i.species {
            Species::IrisSetosa => setosa += 1,
            Species::IrisVersicolor => versicolor += 1,
            Species::IrisVirginica => virginica += 1,
        }
    }

    assert_eq!(setosa, 50);
    assert_eq!(versicolor, 50);
    assert_eq!(virginica, 50);
}
