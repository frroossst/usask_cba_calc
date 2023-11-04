use super::clo::CLO;

/// Type A = no pass requirement
/// Type B = 70% minimum
/// Type B+ = 50% minimum
/// Type C = no pass requirement
#[derive(Debug, Clone, PartialEq)]
pub enum DifficultyType {
    TypeA,
    TypeB,
    TypeBPlus,
    TypeC,
}

#[derive(Debug, Clone)]
pub struct Subject {
    pub name: String,
    clos: Vec<CLO>,
    pub current_grade: Result<f32, f32>,
}

impl Subject {
    pub fn new(name: String) -> Subject {
        Subject { name, clos: Vec::new(), current_grade: Err(f32::NAN) }
    }

    pub fn add_clo(&mut self, name: f32, difficulty_type: DifficultyType, weight_in_subject: f32) {
        let clo_to_be_added = CLO::new(name, difficulty_type, weight_in_subject);
        self.clos.push(clo_to_be_added);
        self.clos.sort();
    }

    pub fn get_clos(&mut self) -> &mut Vec<CLO> {
        &mut self.clos
    }

    /// calculate the weighted average of each of the CLOs
    /// and determine if the class is passed or not
    pub fn get_subject_grade(&mut self) -> f32 {
        let mut grade = 0.0;
        for i in self.get_clos() {
            let clo_grade = i.get_clo_grade();
            let clo_weight = i.get_clo_weight() * 0.01;

            grade += clo_grade * clo_weight;
        }
        self.current_grade = self.get_if_subject_pass(grade);
        grade
    }

    fn get_if_subject_pass(&self, grade: f32) -> Result<f32, f32> {
        if grade < 50.0 {
            Err(grade)
        } else {
            Ok(grade)
        }
    }
}

