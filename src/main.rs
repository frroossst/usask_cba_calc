use grade_calculator::ingest::*;

fn main() {
    let subjects = read_and_parse_file("./grades.json".to_string());

    println!("{:#?}", subjects);

    for i in subjects.unwrap().into_iter() {
        println!("Subject: {}, Grade: {}", i.name, i.get_subject_grade());
    }
}
