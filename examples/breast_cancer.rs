use data_science::datasets::breast_cancer::BreastCancer;

fn main() {
    println!("Num features: {}", BreastCancer::NUM_FEATURES);
    println!("Feature names: {}", BreastCancer::FEATURE_NAMES.join(", "));
    println!("Target names: {}", BreastCancer::TARGET_NAMES.join(", "));

    println!("Num rows: {}", BreastCancer::NUM_ROWS);

    let data = BreastCancer::get_as_vec();
    let row = data.get(0).unwrap();
    println!(
        "First five features + target of a row: {}, {}, {}, {}, {}, ..., {}",
        row.radius_mean,
        row.texture_mean,
        row.perimeter_mean,
        row.area_mean,
        row.smoothness_mean,
        row.target
    );
}
