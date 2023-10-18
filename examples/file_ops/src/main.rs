use std::fs;

const IN_FILE_PATH: &str = "./data/data_in.csv";
const OUT_FILE_PATH: &str = "./data/data_out.csv";

fn main() {
    process_csv_as_string();
}

fn process_csv_as_string() {
    let csv_contents = fs::read_to_string(IN_FILE_PATH)
        .expect("Could not read CSV file.");
    println!("{}", csv_contents);

    fs::write(OUT_FILE_PATH, csv_contents).expect("Unable to write CSV.");
}