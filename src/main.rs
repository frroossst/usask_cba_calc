use usask_cba_calc::ingest::*;
use std::io::{self, Read};



fn main() {
    let args: Vec<String> = std::env::args().collect();

    let subjects;

    if args.len() == 2 {
            if args.last().unwrap() == "-h" || args.last().unwrap() == "--help" {
            eprintln!("Usage: usask-cba-calc [file_path]\n");
            eprintln!("Arguments:");
            eprintln!("    [file_path]  Path to file containing JSON data.\n");
            eprintln!("If no file path is provided, the program will read from stdin.\n\n");
            eprintln!("{}\n", env!("CARGO_PKG_DESCRIPTION"));
            eprintln!("usask-cba-calc v{}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }
        // If exactly one argument is provided, treat it as a file path.
        let file_path = &args[1];
        subjects = read_and_parse_file(file_path.to_string());
    } else {
        // If no or more than one argument is provided, read from stdin (piped input).
        let mut input_data = String::new();
        io::stdin().read_to_string(&mut input_data).expect("Failed to read from stdin");
        subjects = populate_json_data(parse_json_data(input_data));
    }

    for i in subjects.unwrap().into_iter() {
        let subject_grade = i.get_subject_grade();
        if subject_grade <= 50.0 {
            println!("Subject: {}, Grade: FAIL", i.name);
        }
        else {
            println!("Subject: {}, Grade: {}", i.name, subject_grade);
        }
    }
}

