use usask_cba_calc::ingest::*;
use std::io::{self, Read};



fn main() {
    let args: Vec<String> = std::env::args().collect();

    let subjects;

    if args.len() == 2 {
        // If exactly one argument is provided, treat it as a file path.
        let file_path = &args[1];
        subjects = read_and_parse_file(file_path.to_string());
    } else {
        // If no or more than one argument is provided, read from stdin (piped input).
        let mut input_data = String::new();
        io::stdin().read_to_string(&mut input_data).expect("Failed to read from stdin");
        subjects = read_and_parse_file(input_data);
    }

    println!("{:#?}", subjects);

    for i in subjects.unwrap().into_iter() {
        println!("Subject: {}, Grade: {}", i.name, i.get_subject_grade());
    }
}
