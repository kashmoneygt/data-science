pub enum IrisError {
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

impl From<csv::Error> for IrisError {
    fn from(e: csv::Error) -> Self {
        Self::CsvError(e)
    }
}
impl From<(u64, &'static str)> for IrisError {
    fn from((linenum, column_name): (u64, &'static str)) -> Self {
        Self::ColumnNotFound {
            linenum,
            column_name,
        }
    }
}

impl<S> From<(u64, &'static str, S)> for IrisError
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
pub struct Iris {
    pub sepal_length_in_cm: f64,
    pub sepal_width_in_cm: f64,
    pub petal_length_in_cm: f64,
    pub petal_width_in_cm: f64,
    pub class: String,
}

impl Iris {
    pub const COLUMNS: [&str; 5] = [
        "sepal_length_in_cm",
        "sepal_width_in_cm",
        "petal_length_in_cm",
        "petal_width_in_cm",
        "class",
    ];

    pub const COLUMN_TYPES: [&str; 5] = [
        "f64",
        "f64",
        "f64",
        "f64",
        "String",
    ];

    pub fn load_csv<P>(filename: P) -> Result<IrisIterator, csv::Error>
    where
        P: AsRef<std::path::Path>,
    {
        let reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_path(filename)?;

        let records = reader.into_records();
        let row = csv::StringRecord::default();
        Ok(IrisIterator { records, row })
    }
}

pub struct IrisIterator {
    records: csv::StringRecordsIntoIter<std::fs::File>,
    row: csv::StringRecord,
}

impl Iterator for IrisIterator {
    type Item = Result<Iris, IrisError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.row = match self.records.next()? {
            Ok(r) => r,
            Err(e) => return Some(Err(e.into())),
        };

        let linenum = self.row.position().unwrap().line();

        let sepal_length_in_cm = match self.row.get(0) {
            None => return Some(Err((linenum, "sepal_length_in_cm").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "sepal_length_in_cm", val).into())),
            },
        };

        let sepal_width_in_cm = match self.row.get(1) {
            None => return Some(Err((linenum, "sepal_width_in_cm").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "sepal_width_in_cm", val).into())),
            },
        };

        let petal_length_in_cm = match self.row.get(2) {
            None => return Some(Err((linenum, "petal_length_in_cm").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "petal_length_in_cm", val).into())),
            },
        };

        let petal_width_in_cm = match self.row.get(3) {
            None => return Some(Err((linenum, "petal_width_in_cm").into())),
            Some(val) => match val.parse() {
                Ok(v) => v,

                Err(_) => return Some(Err((linenum, "petal_width_in_cm", val).into())),
            },
        };

        let class = match self.row.get(4) {
            None => return Some(Err((linenum, "class").into())),
            Some(val) => val.to_owned(),
        };

        let res = Iris {
            sepal_length_in_cm,
            sepal_width_in_cm,
            petal_length_in_cm,
            petal_width_in_cm,
            class,
        };

        Some(Ok(res))
    }
}

fn main() {
    for row in Iris::load_csv("examples/iris.csv")
        .expect("Couldn't load file")
        .flatten()
    {
        println!("Got row: {row:?}");
    }
}
