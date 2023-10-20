use serde_json::Value;
use crate::subject::{Subject, self};
use core::fmt;
use std::{fs::File, io::Read};
use std::error::Error;

#[derive(Debug)]
pub struct SchemaError {
    desc: String
}

impl SchemaError {
    fn new(desc: &str) -> SchemaError {
        SchemaError { desc: desc.to_string() }
    }
}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Schema Error: {}", self.desc)
    }
}

impl Error for SchemaError {}

pub type SchemaResult<T> = std::result::Result<T, SchemaError>;

pub fn read_and_parse_file(file_path: String) -> SchemaResult<()> {
    let mut fobj = match File::open(file_path.clone()) {
        Ok(fobj) => fobj,
        Err(e) => panic!("Error opening file: {}", e),
    };

    let mut content = String::new();

    fobj.read_to_string(&mut content).expect("Error reading file");

    populate_json_data(parse_json_data(content));

    Ok(())
}

/// parse json from string and return a workable 
/// type 
fn parse_json_data(content: String) -> Value {
    return serde_json::from_str(content.as_str()).expect("Error parsing json");
}

/*
    let mut subject = Subject::new("MY101".to_string());
    subject.add_clo(1.1, DifficultyType::TypeB, 100.0);
    subject.get_clos()[0].add_rlo(1.1, 100.0);
    subject.get_clos()[0].get_rlos()[0].add_assignment_grade(100.0);
 */
pub fn populate_json_data(parsed_data: Value) -> SchemaResult<()> {
    if let Value::Object(obj) = parsed_data {
        // Get all keys as a vector
        let subject_keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();

        for subject_key in subject_keys {
            println!("{:?}", subject_key)
        }

        Ok(())
    }
    else {
        Err(SchemaError::new("unable to index into JSON keys"))
    }
}

