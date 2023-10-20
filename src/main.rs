use grade_calculator::subject::{Subject, DifficultyType};
use grade_calculator::ingest::*;

fn main() {
    println!("Hello, world!");

    let subjects = read_and_parse_file("./grades.json".to_string());
}
