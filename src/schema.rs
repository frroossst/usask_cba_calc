use ansi_term::{Colour, Style};
use serde_json::{Map, Value};
use std::io::{self, Write};
use std::fs::File;

fn input() -> String {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).expect("Failed to read line");
    return input_buffer.trim().to_string();
}

enum PrintSettings {
    Style(Style),
    Colour(Colour),
}

struct PrintArgs<'a> {
    str_to_print: &'a str,
    arg: Option<&'a str>,
    colour: Option<PrintSettings>,
}

fn prompt(args: PrintArgs) {
    let PrintArgs { str_to_print, arg, colour } = args;
    match (arg, colour) {
        (Some(arg), Some(PrintSettings::Colour(colour))) => print!("[{}] {} ", colour.paint(arg), colour.paint(str_to_print)),
        (Some(arg), Some(PrintSettings::Style(style))) => print!("[{}] {} ", style.paint(arg), style.paint(str_to_print)),
        (None, Some(PrintSettings::Colour(colour))) => print!("{}", colour.paint(str_to_print)),
        (None, Some(PrintSettings::Style(style))) => print!("{}", style.paint(str_to_print)),
        (Some(arg), None) => print!("[{}] {} ", arg, str_to_print),
        (None, None) => print!("{}", str_to_print),
    }
    io::stdout().flush().unwrap();
}

pub fn construct_schema() {
    let mut schema = Map::new();

    loop {
        prompt(PrintArgs { 
            str_to_print: "Enter subject name or 'q' to quit: ", 
            arg: None, 
            colour: Some(PrintSettings::Style(Style::new().underline().fg(Colour::Yellow))),
        });
        let subject_name = input();
        if subject_name == "q" {
            break;
        }

        let mut clos = Map::new();

        loop {
            prompt(PrintArgs { 
                str_to_print: "Enter CLO name (1.1, 3.2 etc.) or 'q' to go back to subjects: ", 
                arg: Some(&subject_name), 
                colour: Some(PrintSettings::Colour(Colour::Cyan)) 
            });
            let clo_name = input();
            if clo_name == "q" {
                break;
            }

            let mut rlos = Map::new();

            loop {
                prompt(PrintArgs { 
                    str_to_print: "Enter 'q' to go back to CLOs: ", 
                    arg: Some(&clo_name), 
                    colour: Some(PrintSettings::Colour(Colour::Cyan))
                });
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
        prompt(PrintArgs { 
            str_to_print: "File already exists. Overwrite? (y/n): ", 
            arg: None,  
            colour: Some(PrintSettings::Colour(Colour::Cyan)) 
        });
        let overwrite = input();
        if overwrite == "y" || overwrite == "Y" {
            fobj = File::create(default_filename).unwrap();
        } else {
            prompt(PrintArgs { 
                str_to_print: "Enter new file name: ", 
                arg: None,  
                colour: Some(PrintSettings::Colour(Colour::Cyan)) 
            });
            let new_file_name = input();
            fobj = File::create(new_file_name).unwrap();
        }
    } else {
        fobj = File::create(default_filename).unwrap();
    }
    serde_json::to_writer_pretty(fobj, &schema).unwrap();
}

