use super::clo::CLO;

/// Type A = no pass requirement
/// Type B = 70% minimum
/// Type B+ = 50% minimum
/// Type C = no pass requirement
#[derive(Debug, Clone)]
pub enum DifficultyType {
    TypeA,
    TypeB,
    TypeBPlus,
    TypeC,
}

pub struct Subject {
    name: String,
    clos: Vec<CLO>,
}

impl Subject {
    pub fn new(name: String) -> Subject {
        Subject { name, clos: Vec::new() }
    }

    pub fn add_clo(&mut self, name: f32, difficulty_type: DifficultyType, weight_in_subject: f32) {
        let clo_to_be_added = CLO::new(name, difficulty_type, weight_in_subject);
        self.clos.push(clo_to_be_added);
    }

    pub fn get_clos(&mut self) -> &mut Vec<CLO> {
        &mut self.clos
    }

    /// calculate the weighted average of each of the CLOs
    /// and determine if the class is passed or not
    pub fn get_subject_grade(&self) -> f32 {
        let mut grade = 0.0;
        for mut i in self.clos.clone() {
            let clo_grade = i.get_clo_grade();
            let clo_weight = i.get_clo_grade() * 0.01;

            grade += clo_grade * clo_weight;
        }

        grade
    }
}

