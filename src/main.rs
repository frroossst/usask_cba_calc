use usask_cba_calc::ingest::*;
use usask_cba_calc::schema::*;
use std::io::{self, Read};
use ansi_term::Color;



fn main() {
    let args: Vec<String> = std::env::args().collect();

    let subjects = if args.len() == 2 {
        if args.last().unwrap() == "-h" || args.last().unwrap() == "--help" {
            eprintln!("Usage: usask-cba-calc [file_path]\n");
            eprintln!("Arguments:");
            eprintln!("    [file_path]  Path to file containing JSON data.\n");
            eprintln!("If no file path is provided, the program will read from stdin.\n");
            eprintln!("Options:");
            eprintln!("    -h, --help   Prints this message");
            eprintln!("    -s, --schema Creates a boilerplate schema");
            eprintln!("\n{}\n", env!("CARGO_PKG_DESCRIPTION"));
            eprintln!("usask-cba-calc v{}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        } else if args.last().unwrap() == "-s" || args.last().unwrap() == "--schema" {
            construct_schema();
            std::process::exit(0);
        }
            // If exactly one argument is provided, treat it as a file path.
            let file_path = &args[1];
            read_and_parse_file(file_path.to_string())
        } else {
            // If no or more than one argument is provided, read from stdin (piped input).
            let mut input_data = String::new();
            io::stdin().read_to_string(&mut input_data).expect("Failed to read from stdin");
            populate_json_data(parse_json_data(input_data))
        };

    let populated_subjects = subjects.unwrap();

    for mut i in populated_subjects.into_iter() {
        let subject_grade = i.get_subject_grade();
        // println!("{:#?}", i);

        if subject_grade <= 50.0 {
            println!("{}: {}", i.name, Color::Red.bold().paint("FAIL"));
        } else if subject_grade <= 60.0 {
            println!("{}: {}", i.name, Color::Yellow.paint(format!("{:.2}", subject_grade)));
        } else if subject_grade <= 100.0 {
            println!("{}: {}", i.name, Color::Green.paint(format!("{:.2}", subject_grade)));
        } else {
            println!("{}: {}", i.name, subject_grade);
        }

        for c in i.get_clos() {
            let clo_header = format!("[CLO {}]:", c.name);
            print!("{}", Color::White.italic().paint(clo_header));
            match c.current_grade {
                Ok(v) => println!(" {}", v),
                Err(e) => println!(" {}", Color::Red.italic().paint(e.to_string())),
            };
            for r in c.get_rlos() {
                let rlo_header = format!("    [RLO {}]:", r.name);
                print!("{}", Color::White.italic().paint(rlo_header));
                match r.current_grade {
                    Ok(v) => println!(" {}", v),
                    Err(e) => println!(" {}", Color::Red.italic().paint(e.to_string())),
                };
            }
        }
        println!("----------------------------------------");

    }
}
