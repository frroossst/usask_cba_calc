use grade_calculator::subject::{Subject, DifficultyType};

fn main() {
    println!("Hello, world!");

    let mut subject = Subject::new("MY101".to_string());

    subject.add_clo(1.1, DifficultyType::TypeB, 100.0);

    subject.get_clos()[0].add_rlo(1.1, 100.0);
    subject.get_clos()[0].get_rlos()[0].add_assignment_grade(100.0);

    let grade = subject.get_subject_grade();
    println!("{:?}", grade);
}
