pub enum BreastCancerError {
    CsvError(csv::Error),
    ColumnNotFound {
        linenum: u64,
        column_name: &'static str,
    },
    InvalidColumnValue {
        linenum: u64,
        column_name: &'static str,
        value: String,
    },
}

impl From<csv::Error> for BreastCancerError {
    fn from(e: csv::Error) -> Self {
        Self::CsvError(e)
    }
}
impl From<(u64, &'static str)> for BreastCancerError {
    fn from((linenum, column_name): (u64, &'static str)) -> Self {
        Self::ColumnNotFound {
            linenum,
            column_name,
        }
    }
}

impl<S> From<(u64, &'static str, S)> for BreastCancerError
where
    S: Into<String>,
{
    fn from((linenum, column_name, value): (u64, &'static str, S)) -> Self {
        Self::InvalidColumnValue {
            linenum,
            column_name,
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BreastCancer {
    pub radius_mean: f64,
    pub texture_mean: f64,
    pub perimeter_mean: f64,
    pub area_mean: f64,
    pub smoothness_mean: f64,
    pub compactness_mean: f64,
    pub concavity_mean: f64,
    pub concave_points_mean: f64,
    pub symmetry_mean: f64,
    pub fractal_dimension_mean: f64,
    pub radius_standard_error: f64,
    pub texture_standard_error: f64,
    pub perimeter_standard_error: f64,
    pub area_standard_error: f64,
    pub smoothness_standard_error: f64,
    pub compactness_standard_error: f64,
    pub concavity_standard_error: f64,
    pub concave_points_standard_error: f64,
    pub symmetry_standard_error: f64,
    pub fractal_dimension_standard_error: f64,
    pub radius_worst: f64,
    pub texture_worst: f64,
    pub perimeter_worst: f64,
    pub area_worst: f64,
    pub smoothness_worst: f64,
    pub compactness_worst: f64,
    pub concavity_worst: f64,
    pub concave_points_worst: f64,
    pub symmetry_worst: f64,
    pub fractal_dimension_worst: f64,
    pub target: u8,
}

impl BreastCancer {
    pub const COLUMNS: [&str; 31] = [
        "radius_mean",
        "texture_mean",
        "perimeter_mean",
        "area_mean",
        "smoothness_mean",
        "compactness_mean",
        "concavity_mean",
        "concave_points_mean",
        "symmetry_mean",
        "fractal_dimension_mean",
        "radius_standard_error",
        "texture_standard_error",
        "perimeter_standard_error",
        "area_standard_error",
        "smoothness_standard_error",
        "compactness_standard_error",
        "concavity_standard_error",
        "concave_points_standard_error",
        "symmetry_standard_error",
        "fractal_dimension_standard_error",
        "radius_worst",
        "texture_worst",
        "perimeter_worst",
        "area_worst",
        "smoothness_worst",
        "compactness_worst",
        "concavity_worst",
        "concave_points_worst",
        "symmetry_worst",
        "fractal_dimension_worst",
        "target",
    ];
    pub const COLUMN_TYPES: [&str; 31] = [
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "f64",
        "u8",
    ];

    pub fn load_csv<P>(filename: P) -> Result<BreastCancerIterator, csv::Error>
    where
        P: AsRef<std::path::Path>,
    {
        let reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_path(filename)?;

        let records = reader.into_records();
        let row = csv::StringRecord::default();
        Ok(BreastCancerIterator { records, row })
    }
}

pub struct BreastCancerIterator {
    records: csv::StringRecordsIntoIter<std::fs::File>,
    row: csv::StringRecord,
}

impl Iterator for BreastCancerIterator {
    type Item = Result<BreastCancer, BreastCancerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.row = match self.records.next()? {
            Ok(r) => r,
            Err(e) => return Some(Err(e.into())),
        };

        let linenum = self.row.position().unwrap().line();

        let radius_mean = match self.row.get(0) {
            None => return Some(Err((linenum, "radius_mean").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "radius_mean", val).into())),
            },
        };

        let texture_mean = match self.row.get(1) {
            None => return Some(Err((linenum, "texture_mean").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "texture_mean", val).into())),
            },
        };

        let perimeter_mean = match self.row.get(2) {
            None => return Some(Err((linenum, "perimeter_mean").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "perimeter_mean", val).into())),
            },
        };

        let area_mean = match self.row.get(3) {
            None => return Some(Err((linenum, "area_mean").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "area_mean", val).into())),
            },
        };

        let smoothness_mean = match self.row.get(4) {
            None => return Some(Err((linenum, "smoothness_mean").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "smoothness_mean", val).into())),
            },
        };

        let compactness_mean = match self.row.get(5) {
            None => return Some(Err((linenum, "compactness_mean").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "compactness_mean", val).into())),
            },
        };

        let concavity_mean = match self.row.get(6) {
            None => return Some(Err((linenum, "concavity_mean").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "concavity_mean", val).into())),
            },
        };

        let concave_points_mean = match self.row.get(7) {
            None => return Some(Err((linenum, "concave_points_mean").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "concave_points_mean", val).into())),
            },
        };

        let symmetry_mean = match self.row.get(8) {
            None => return Some(Err((linenum, "symmetry_mean").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "symmetry_mean", val).into())),
            },
        };

        let fractal_dimension_mean = match self.row.get(9) {
            None => return Some(Err((linenum, "fractal_dimension_mean").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "fractal_dimension_mean", val).into())),
            },
        };

        let radius_standard_error = match self.row.get(10) {
            None => return Some(Err((linenum, "radius_standard_error").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "radius_standard_error", val).into())),
            },
        };

        let texture_standard_error = match self.row.get(11) {
            None => return Some(Err((linenum, "texture_standard_error").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "texture_standard_error", val).into())),
            },
        };

        let perimeter_standard_error = match self.row.get(12) {
            None => return Some(Err((linenum, "perimeter_standard_error").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "perimeter_standard_error", val).into())),
            },
        };

        let area_standard_error = match self.row.get(13) {
            None => return Some(Err((linenum, "area_standard_error").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "area_standard_error", val).into())),
            },
        };

        let smoothness_standard_error = match self.row.get(14) {
            None => return Some(Err((linenum, "smoothness_standard_error").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "smoothness_standard_error", val).into())),
            },
        };

        let compactness_standard_error = match self.row.get(15) {
            None => return Some(Err((linenum, "compactness_standard_error").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "compactness_standard_error", val).into())),
            },
        };

        let concavity_standard_error = match self.row.get(16) {
            None => return Some(Err((linenum, "concavity_standard_error").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "concavity_standard_error", val).into())),
            },
        };

        let concave_points_standard_error = match self.row.get(17) {
            None => return Some(Err((linenum, "concave_points_standard_error").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "concave_points_standard_error", val).into())),
            },
        };

        let symmetry_standard_error = match self.row.get(18) {
            None => return Some(Err((linenum, "symmetry_standard_error").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "symmetry_standard_error", val).into())),
            },
        };

        let fractal_dimension_standard_error = match self.row.get(19) {
            None => return Some(Err((linenum, "fractal_dimension_standard_error").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "fractal_dimension_standard_error", val).into())),
            },
        };

        let radius_worst = match self.row.get(20) {
            None => return Some(Err((linenum, "radius_worst").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "radius_worst", val).into())),
            },
        };

        let texture_worst = match self.row.get(21) {
            None => return Some(Err((linenum, "texture_worst").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "texture_worst", val).into())),
            },
        };

        let perimeter_worst = match self.row.get(22) {
            None => return Some(Err((linenum, "perimeter_worst").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "perimeter_worst", val).into())),
            },
        };

        let area_worst = match self.row.get(23) {
            None => return Some(Err((linenum, "area_worst").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "area_worst", val).into())),
            },
        };

        let smoothness_worst = match self.row.get(24) {
            None => return Some(Err((linenum, "smoothness_worst").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "smoothness_worst", val).into())),
            },
        };

        let compactness_worst = match self.row.get(25) {
            None => return Some(Err((linenum, "compactness_worst").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "compactness_worst", val).into())),
            },
        };

        let concavity_worst = match self.row.get(26) {
            None => return Some(Err((linenum, "concavity_worst").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "concavity_worst", val).into())),
            },
        };

        let concave_points_worst = match self.row.get(27) {
            None => return Some(Err((linenum, "concave_points_worst").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "concave_points_worst", val).into())),
            },
        };

        let symmetry_worst = match self.row.get(28) {
            None => return Some(Err((linenum, "symmetry_worst").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "symmetry_worst", val).into())),
            },
        };

        let fractal_dimension_worst = match self.row.get(29) {
            None => return Some(Err((linenum, "fractal_dimension_worst").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "fractal_dimension_worst", val).into())),
            },
        };

        let target = match self.row.get(30) {
            None => return Some(Err((linenum, "target").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "target", val).into())),
            },
        };

        let res = BreastCancer {
            radius_mean,
            texture_mean,
            perimeter_mean,
            area_mean,
            smoothness_mean,
            compactness_mean,
            concavity_mean,
            concave_points_mean,
            symmetry_mean,
            fractal_dimension_mean,
            radius_standard_error,
            texture_standard_error,
            perimeter_standard_error,
            area_standard_error,
            smoothness_standard_error,
            compactness_standard_error,
            concavity_standard_error,
            concave_points_standard_error,
            symmetry_standard_error,
            fractal_dimension_standard_error,
            radius_worst,
            texture_worst,
            perimeter_worst,
            area_worst,
            smoothness_worst,
            compactness_worst,
            concavity_worst,
            concave_points_worst,
            symmetry_worst,
            fractal_dimension_worst,
            target,
        };

        Some(Ok(res))
    }
}

fn main() {
    for row in BreastCancer::load_csv("breast_cancer.csv")
        .expect("Couldn't load file")
        .flatten()
    {
        println!("Got row: {row:?}");
    }
}
