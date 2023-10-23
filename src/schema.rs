use serde_json::{Map, Value};
use std::{fs::File, io::Read};
use std::io::{self, Write};


/// Just returns String from stdin like Python's input()
fn input() -> String {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).expect("Failed to read line");
    return input_buffer.trim().to_string();
}

/// Just so that I don't have to flush stdout everytime
fn prompt(str_to_print: &str) {
    print!("{}", str_to_print);
    io::stdout().flush().unwrap();
}


pub fn construct_schema() {
    let mut schema = Map::new();

    prompt("Enter numebr of subjects: ");
    let number_of_subjects = match input().parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            panic!("Please enter a valid number");
        }
    };

    for _ in 0..number_of_subjects {
        prompt("Enter subject name: ");
        let subject_name = input();
        let mut subject_schema: Value = Default::default();
    
        prompt("Enter number of CLOs: ");
        let number_of_clos = match input().parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                panic!("Please enter a valid number");
            }
        };

        for _ in 0..number_of_clos {
            prompt("Enter CLO: ");
            let clo = input();
            subject_schema.as_object_mut().unwrap().get_mut("CLOs").unwrap().as_array_mut().unwrap().push(Value::String(clo));
        }


        subject_schema.as_object_mut().unwrap().insert("CLOs".to_string(), Value::Array(vec![]));



        schema.insert(subject_name, subject_schema);

    }

    let fobj = match File::create("./schema.json") {
        Ok(f) => f,
        Err(_) => {
            panic!("Failed to open schema.json");
        }
    };

    serde_json::to_writer_pretty(fobj, &schema).unwrap();

}
