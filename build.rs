use std::{env, fs::File, io::Write};

mod generated_types;
use generated_types::breast_cancer::BreastCancer;
use generated_types::iris::Iris;

fn main() {
    let cur_dir = env::current_dir().unwrap();
    let cur_dir = cur_dir.to_str().unwrap();

    let src_datasets = format!("{cur_dir}/src/datasets");
    let raw_data = format!("{cur_dir}/raw_data");

    macro_rules! generate_data_file {
        ($struct_name: ident, $filename: literal) => {
            // TODO: instead of writing datasets/iris.rs,
            // 1. create datasets/iris/ directory if not exists,
            // 2. stub in a mod.rs file if not exists, and
            // 3. generate (below) datasets/iris/iris.rs
            let out_file = format!("{src_datasets}/{}/{}.rs", $filename, $filename);
            let out_file = File::create(&out_file).unwrap();
            let mut buf = std::io::BufWriter::new(out_file);

            writeln!(buf, "#[derive(Clone, Debug)]").ok();
            writeln!(buf, "pub struct {} {{", stringify!($struct_name)).ok();

            for (cname, ctype) in $struct_name::COLUMNS.iter().zip($struct_name::COLUMN_TYPES) {
                if ctype == "String" {
                    writeln!(buf, "    pub {cname}: &'static str,").ok();
                } else {
                    writeln!(buf, "    pub {cname}: {ctype},").ok();
                }
            }
            writeln!(buf, "}}").ok();

            let data = $struct_name::load_csv(&format!("{raw_data}/{}.csv", $filename))
                .expect(&format!("Couldn't load {}.csv", $filename))
                .flatten()
                .collect::<Vec<_>>();

            writeln!(
                buf,
                "pub const DATA: [{}; {}] = [",
                stringify!($struct_name),
                data.len()
            )
            .ok();

            for row in data {
                writeln!(buf, "    {row:?},").ok();
            }
            writeln!(buf, "];").ok();
        };
    }

    generate_data_file!(Iris, "iris");
    generate_data_file!(BreastCancer, "breast_cancer");
}
