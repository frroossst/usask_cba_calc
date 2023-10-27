use usask_cba_calc::ingest::*;
use usask_cba_calc::schema::*;
use usask_cba_calc::misc::*;
use tokio::time::Duration;
use ansi_term::Color;



#[tokio::main]
async fn main() -> () {
    let args: Vec<String> = std::env::args().collect();

    let subjects = if args.len() == 2 {
        if args.last().unwrap() == "-h" || args.last().unwrap() == "--help" {
            print_help_message();
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
            let input_timeout = Duration::from_millis(2750);
            let input_data = match read_stdin_with_timeout(input_timeout).await {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}", Color::Yellow.underline().paint(format!("{}", e)));
                    print_help_message();
                    std::process::exit(124);
                },
            };
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
                Ok(v) => println!(" {:.2}", v),
                Err(e) => println!(" {}", Color::Red.italic().paint(format!("{:.2}", e.to_string()))),
            };
            for r in c.get_rlos() {
                let rlo_header = format!("    [RLO {}]:", r.name);
                print!("{}", Color::White.italic().paint(rlo_header));
                match r.current_grade {
                    Ok(v) => println!(" {:.2}", v),
                    Err(e) => println!(" {}", Color::Red.italic().paint(format!("{:.2}", e.to_string()))),
                };
            }
        }
        println!("----------------------------------------");

    }
}
