pub trait Scalable {
    const COLUMN_COUNT: usize;
    fn get_as_f64(&self, index: usize) -> Option<f64>;
    fn set_f64(&mut self, index: usize, value: f64);
}

pub fn scale_subtract_mean<S>(input_data: &mut [S])
where
    S: Scalable,
{
    // store the sum of values
    let mut sum = (0..S::COLUMN_COUNT).map(|_| 0.0).collect::<Vec<_>>();

    for row in input_data.iter() {
        for column in 0..S::COLUMN_COUNT {
            if let Some(val) = row.get_as_f64(column) {
                sum[column] += val;
            }
        }
    }

    // scale down the sum
    let n = input_data.len() as f64;
    let avg = sum.iter().map(|s| s / n).collect::<Vec<_>>();

    for row in input_data.iter_mut() {
        for column in 0..S::COLUMN_COUNT {
            if let Some(val) = row.get_as_f64(column) {
                let scaled = val - avg[column];
                row.set_f64(column, scaled);
            }
        }
    }
}
