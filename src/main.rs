use usask_cba_calc::ingest::*;
use usask_cba_calc::schema::*;
use usask_cba_calc::misc::*;
use tokio::time::Duration;
use ansi_term::Color;



#[tokio::main]
async fn main() -> () {
    let src = r#"{
    "SampleTest" : {
        "CLOs": {
            "0.1" : {
                "difficulty_type": "B",
                "weightage": 100.0,
                "RLOs": {
                    "0.2": {
                        "weightage": 100.0,
                        "assignments": [100, 0]
                    }
                }
            }
        }
    },
    "Subject1": {
        "CLOs": {
            "1.1": {
                "difficulty_type": "B",
                "weightage": 20.0,
                "RLOs": {
                    "1.3": {
                        "weightage": 50.0,
                        "assignments": [95, 90, 85]
                    },
                    "3.3": {
                        "weightage": 50.0,
                        "assignments": [80, 75]
                    }
                }
            },
            "1.2": {
                "difficulty_type": "B+",
                "weightage": 80.0,
                "RLOs": {
                    "1.1": {
                        "weightage": 100.0,
                        "assignments": [95, 90, 85]
                    }
                }
            }
        }
    },
    "Subject2": {
        "CLOs": {
            "2.1": {
                "difficulty_type": "A",
                "weightage": 0.0,
                "RLOs": {
                    "2.1": {
                        "weightage": 100.0,
                        "assignments": [12]
                    }
                }
            },
            "1.2": {
                "difficulty_type": "B+",
                "weightage": 100.0,
                "RLOs": {
                    "2.2": {
                        "weightage": 100.0,
                        "assignments": [95, 0, 0]
                    }
                }
            }
        }
    }
}
"#.to_string();

    let subjects = populate_json_data(parse_json_data(src));

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
                Err(e) => {
                    println!(" {}", Color::Red.italic().paint(format!("{:.2}", e.to_string())));
                },
            };
            for r in c.get_rlos() {
                let rlo_header = format!("    [RLO {}]:", r.name);
                print!("{}", Color::White.italic().paint(rlo_header));
                match r.current_grade {
                    Ok(v) => println!(" {:.2}", v),
                    Err(e) => {
                        let rounded_grade = format!("{:.2}", e.to_string());
                        println!(" {}", Color::Red.italic().paint(rounded_grade));
                    }
                };
            }
        }
        println!("----------------------------------------");

    }
}
