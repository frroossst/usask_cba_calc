use serde_json::{Map, Value};
use std::io::{self, Write};
use std::fs::File;

fn input() -> String {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).expect("Failed to read line");
    return input_buffer.trim().to_string();
}

fn prompt(str_to_print: &str, arg: Option<&str>) {
    match arg {
        Some(s) => print!("[{}] {}", s, str_to_print),
        None => print!("{}", str_to_print),
    }
    io::stdout().flush().unwrap();
}

pub fn construct_schema() {
    let mut schema = Map::new();

    loop {
        prompt("Enter subject name or 'q' to quit: ", None);
        let subject_name = input();
        if subject_name == "q" {
            break;
        }

        let mut clos = Map::new();

        loop {
            prompt("Enter CLO name (1.1, 3.2 etc.) or 'q' to go back to subjects: ", Some(&subject_name));
            let clo_name = input();
            if clo_name == "q" {
                break;
            }

            let mut rlos = Map::new();

            loop {
                prompt("Enter RLO name (1.1, 3.2 etc.) or 'q' to go back to CLOs: ", Some(&clo_name));
                let rlo_name = input();
                if rlo_name == "q" {
                    break;
                }

                rlos.insert(rlo_name.clone(), Value::Object(Map::new()));
                rlos.get_mut(&rlo_name).unwrap().as_object_mut().unwrap().insert("weightage".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap()));
                rlos.get_mut(&rlo_name).unwrap().as_object_mut().unwrap().insert("assignments".to_string(), Value::Array(vec![]));
            }

            clos.insert(clo_name.clone(), Value::Object(Map::new()));
            clos.get_mut(&clo_name).unwrap().as_object_mut().unwrap().insert("difficulty_type".to_string(), Value::String(String::from("")));
            clos.get_mut(&clo_name).unwrap().as_object_mut().unwrap().insert("weightage".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap()));
            clos.get_mut(&clo_name).unwrap().as_object_mut().unwrap().insert("RLOs".to_string(), Value::Object(rlos));
        }

        schema.insert(subject_name.clone(), Value::Object(Map::new()));
        schema.get_mut(&subject_name).unwrap().as_object_mut().unwrap().insert("CLOs".to_string(), Value::Object(clos));
    }

    let fobj;
    let default_filename = "grades.json";

    if std::path::Path::new(default_filename).exists() {
        prompt("File already exists. Overwrite? (y/n): ", None);
        let overwrite = input();
        if overwrite == "y" || overwrite == "Y" {
            fobj = File::create(default_filename).unwrap();
        } else {
            prompt("Enter new file name: ", None);
            let new_file_name = input();
            fobj = File::create(new_file_name).unwrap();
        }
    } else {
        fobj = File::create(default_filename).unwrap();
    }
    serde_json::to_writer_pretty(fobj, &schema).unwrap();
}

