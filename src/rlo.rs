use super::subject::DifficultyType;
use crate::weighted_rlo_grade;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct RLO {
    pub name: f32,
    difficulty_type: DifficultyType,
    /// The assignment grades are individual
    /// percentages as entered by the user
    assignment_grades: Vec<f32>,
    weight_in_clo: f32,
    pub current_grade: Result<f32, f32>,
}

impl RLO {
    pub fn new(name: f32, difficulty_type: DifficultyType, weight_in_clo: f32) -> Self {
        RLO {
            name,
            difficulty_type,
            assignment_grades: Vec::new(),
            weight_in_clo,
            current_grade: Err(f32::NAN),
        }
    }

    pub fn add_assignment_grade(&mut self, grade: f32) {
        self.assignment_grades.push(grade);
    }

    /// The grade for each RLO is calculated iteratively
    /// A new higher grade replaces an original lower grade
    /// But a lower grade than the current calculated grade
    /// causes the new grade to be the average of the current
    /// calculated grade and the new grade
    pub fn get_rlo_grade(&mut self) -> f32 {
        let mut grade: f32 = 0.0;

        // Simple average for type Cs
        if self.difficulty_type == DifficultyType::TypeC {
            let mut total: f32 = 0.0;
            for i in self.assignment_grades.clone() {
                total += i
            }
            return total / self.assignment_grades.len() as f32;
        } else {
            for assignment_grade in self.assignment_grades.clone() {
                if assignment_grade > grade {
                    grade = assignment_grade;
                } else {
                    grade = weighted_rlo_grade!(grade, assignment_grade);
                }
            }
            self.current_grade = self.get_if_rlo_pass(grade);
            grade
        }
    }

    fn get_if_rlo_pass(&self, grade: f32) -> Result<f32, f32> {
        match self.difficulty_type {
            DifficultyType::TypeB => {
                if grade < 75.0 {
                    Err(grade)
                } else {
                    Ok(grade)
                }
            }
            DifficultyType::TypeBPlus => {
                if grade < 50.0 {
                    Err(grade)
                } else {
                    Ok(grade)
                }
            }
            _ => Ok(grade),
        }
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
