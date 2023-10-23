use super::subject::{DifficultyType, Subject};
use std::{fs::File, io::Read};
use serde_json::Value;
use std::error::Error;
use core::fmt;


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

pub fn read_and_parse_file(file_path: String) -> Result<Box<Vec<Subject>>, SchemaError> {
    let mut fobj = match File::open(file_path.clone()) {
        Ok(fobj) => fobj,
        Err(e) => panic!("Error opening file: {}", e),
    };

    let mut content = String::new();

    fobj.read_to_string(&mut content).expect("Error reading file");

    populate_json_data(parse_json_data(content))
}

/// parse json from string and return a workable 
/// type 
pub fn parse_json_data(content: String) -> Value {
    return serde_json::from_str(content.as_str()).expect("Error parsing json");
}

/*
    let mut subject = Subject::new("MY101".to_string());
    subject.add_clo(1.1, DifficultyType::TypeB, 100.0);
    subject.get_clos()[0].add_rlo(1.1, 100.0);
    subject.get_clos()[0].get_rlos()[0].add_assignment_grade(100.0);
 */
pub fn populate_json_data(parsed_data: Value) -> SchemaResult<Box<Vec<Subject>>> {
    if let Value::Object(ref obj) = parsed_data {
        let mut subjects: Vec<Subject> = Vec::new();
        // Get all keys as a vector
        let subject_keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();

        // get all subjects
        for subject_key in subject_keys {
            let mut curr_sub = Subject::new(subject_key.to_string());
            // get all clos
            let clos = obj.get(subject_key).unwrap().as_object().unwrap().get("CLOs").unwrap().as_object().unwrap();
            for clo_key in clos {
                let curr_clo_name = clo_key.0.parse::<f32>().unwrap();
                let curr_clo_difficulty_type = match clo_key.1.get("difficulty_type").unwrap().as_str() {
                    Some("A") => DifficultyType::TypeA,
                    Some("B") => DifficultyType::TypeB,
                    Some("B+") => DifficultyType::TypeBPlus,
                    Some("C") => DifficultyType::TypeC,
                    Some(_) => panic!("Error parsing difficulty type"),
                    None => panic!("Error parsing difficulty type"),
                };
                let curr_clo_weight = clo_key.1.get("weightage").unwrap().as_f64().unwrap() as f32;
                curr_sub.add_clo(curr_clo_name, curr_clo_difficulty_type, curr_clo_weight);

                // get all rlos
                let rlos = clo_key.1.get("RLOs").unwrap().as_object().unwrap();
                for rlo_key in rlos {
                    let curr_rlo_name = rlo_key.0.parse::<f32>().unwrap();
                    let curr_rlo_weight = rlo_key.1.get("weightage").unwrap().as_f64().unwrap() as f32;

                    for i in curr_sub.get_clos() {
                        if i.name == curr_clo_name {
                            i.add_rlo(curr_rlo_name, curr_rlo_weight);
                            for j in i.get_rlos() {
                                if j.name == curr_rlo_name {
                                    let curr_rlo_assignments = rlo_key.1.get("assignments").unwrap().as_array().unwrap();
                                    for k in curr_rlo_assignments {
                                        j.add_assignment_grade(k.as_f64().unwrap() as f32);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            subjects.push(curr_sub);
        }

        Ok(Box::new(subjects))
    }
    else {
        Err(SchemaError::new("unable to index into JSON keys"))
    }
}
