use serde_json::{Map, Value};
use std::io::{self, Write};
use std::fs::File;

fn input() -> String {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).expect("Failed to read line");
    return input_buffer.trim().to_string();
}

fn prompt(str_to_print: &str) {
    print!("{}", str_to_print);
    io::stdout().flush().unwrap();
}

pub fn construct_schema() {
    let mut schema = Map::new();

    prompt("Enter number of subjects: ");
    let number_of_subjects: u32 = input().trim().parse().expect("Please enter a valid number");

    for _ in 0..number_of_subjects {
        prompt("Enter subject name: ");
        let subject_name = input();

        prompt("Enter number of CLOs: ");
        let number_of_clos: u32 = input().trim().parse().expect("Please enter a valid number");

        let mut clos = Map::new();

        for _ in 0..number_of_clos {
            prompt("Enter CLO name (1.1, 3.2 etc.): ");
            let clo_name = input();

            prompt("Enter number of RLOs: ");
            let number_of_rlos: u32 = input().trim().parse().expect("Please enter a valid number");

            let mut rlos = Map::new();

            for _ in 0..number_of_rlos {
                prompt("Enter RLO name (1.1, 3.2 etc.): ");
                let rlo_name = input();

                rlos.insert(rlo_name.clone(), Value::Object(Map::new()));
                // rlos.get_mut(&rlo_name).unwrap().as_object_mut().unwrap().insert("difficulty_type".to_string(), Value::String(String::from("")));
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
    // check if the file grades.json exists, if it does, ask if it's okay to overwrite
    // if it doesn't, create it
    // if it does and the user says yes, overwrite it
    // if it does and the user says no, then ask for a new file name
    if std::path::Path::new(default_filename).exists() {
        prompt("file already exists. Overwrite? (y/n): ");
        let overwrite = input();
        if overwrite == "y" || overwrite == "Y" {
            fobj = File::create(default_filename).unwrap();
        } else {
            prompt("Enter new file name: ");
            let new_file_name = input();
            fobj = File::create(new_file_name).unwrap();
        }
    } else {
        fobj = File::create(default_filename).unwrap();
    }
    serde_json::to_writer_pretty(fobj, &schema).unwrap();

}

