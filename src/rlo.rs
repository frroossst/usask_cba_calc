use super::subject::DifficultyType;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct RLO {
    pub name: f32,
    difficulty_type: DifficultyType,
    /// The assignment grades are individual 
    /// percentages as entered by the user
    assignment_grades: Vec<f32>,
    weight_in_clo: f32,
}

impl RLO {
    pub fn new(name: f32, difficulty_type: DifficultyType, weight_in_clo: f32) -> Self {
        RLO { name, difficulty_type, assignment_grades: Vec::new(), weight_in_clo }
    }

    pub fn add_assignment_grade(&mut self, grade: f32) {
        self.assignment_grades.push(grade);
    }

    /// The grade for each RLO is calculated iteratively
    /// A new higher grade replaces an original lower grade
    /// But a lower grade than the current calculated grade
    /// causes the new grade to be the average of the current
    /// calculated grade and the new grade
    pub fn get_rlo_grade(&self) -> f32 {
        let mut grade: f32 = 0.0;
        for assignment_grade in self.assignment_grades.clone() {
            if assignment_grade > grade {
                grade = assignment_grade;
            } else {
                // TODO: replace with weighted average per class
                grade = (grade + assignment_grade) / 2.0;
            }
        }
        println!("rlo: {:?} has grade: {:?}", self.name, grade);
        grade
    }

    pub fn get_rlo_weight(&self) -> f32 {
        self.weight_in_clo
    }
}

impl Ord for RLO {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for RLO {}

impl PartialEq for RLO {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }

}

impl PartialOrd for RLO {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

