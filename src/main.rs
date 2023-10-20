use grade_calculator::subject::{Subject, DifficultyType};
use grade_calculator::ingest::read_and_parse_file;

fn main() {
    println!("Hello, world!");

    let parsed_data = read_and_parse_file("./grades.toml".to_string());

    println!("{:?}", parsed_data);

    let mut subject = Subject::new("MY101".to_string());

    subject.add_clo(1.1, DifficultyType::TypeB, 100.0);

    subject.get_clos()[0].add_rlo(1.1, 100.0);
    subject.get_clos()[0].get_rlos()[0].add_assignment_grade(100.0);

    let grade = subject.get_subject_grade();
    println!("{:?}", grade);
}
