pub struct Diabetes {
    pub age: i32,
    pub sex: Sex,
    pub bmi: f32,
    pub bp: f32,
    pub tc: f32,
    pub ldl: f32,
    pub hdl: f32,
    pub tch: f32,
    pub ltg: f32,
    pub glu: f32,

    pub target: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sex {
    
}

impl Diabetes {
    pub const NUM_FEATURES: usize = 10;
    pub const FEATURE_NAMES: [&str; Self::NUM_FEATURES] = [
        "age (in years)",
        "sex",
        "bmi (body mass index)",
        "bp (blood pressure)",
        "tc (total serum cholesterol)",
        "ldl (low-density lipoproteins)",
        "hdl (high-density lipoproteins)",
        "tch (total cholesterol / HDL)",
        "ltg (possibly log of serum triglycerides level)",
        "glu (blood sugar level)",
    ];
}

macro_rules! diabetes_row {
    ($age:literal, $sex:literal, $bmi:literal, $bp:literal, $tc:literal, $ldl:literal, $hdl:literal, $tch:literal, $ltg:literal, $glu:literal, $target: literal) => {
        Diabetes {
            age: $age,
            sex: match $sex {

            },
            bmi: $bmi,
            bp: $bp,
            tc: $tc,
            ldl: $ldl,
            hdl: $hdl,
            tch: $tch,
            ltg: $ltg,
            glu: $glu,

            target: $target,
        }
    };
}

pub const DATA: [Diabetes; 442] = [
    diabetes_row!(59, 2, 32.1, 101.0, 157, 93.2, 38.0, 4.0, 4.8598, 87, 1.510000000000000000e+02)
]
