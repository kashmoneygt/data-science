use data_science::datasets::iris::Iris;

fn main() {
    println!("Num features: {}", Iris::NUM_FEATURES);
    println!("Feature names: {}", Iris::FEATURE_NAMES.join(", "));

    println!("Num rows: {}", Iris::NUM_ROWS);

    let data = Iris::get_as_vec();
    let row = data.get(0).unwrap();
    println!(
        "Example row: {}, {}, {}, {}, {}",
        row.sepal_length_in_cm,
        row.sepal_width_in_cm,
        row.petal_length_in_cm,
        row.petal_width_in_cm,
        row.class
    );
}
