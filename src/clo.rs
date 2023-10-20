use super::subject::DifficultyType;
use super::rlo::RLO;

#[derive(Debug, Clone)]
pub struct CLO {
    pub name: f32,
    difficulty_type: DifficultyType,
    rlos: Vec<RLO>,
    weight_in_subject: f32,
}

impl CLO {
    pub fn new(name: f32, difficulty_type: DifficultyType, weight_in_subject: f32) -> Self {
        CLO { name, difficulty_type, rlos: Vec::new(), weight_in_subject }
    }

    pub fn add_rlo(&mut self, name: f32, weight_in_clo: f32) {
        let rlo_to_be_added = RLO::new(name, self.difficulty_type.clone(), weight_in_clo);
        self.rlos.push(rlo_to_be_added);
    }

    pub fn get_rlos(&mut self) -> &mut Vec<RLO> {
        &mut self.rlos
    }

    /// Simple weighted average of each child RLO
    pub fn get_clo_grade(&mut self) -> f32 {
        let mut grade = 0.0;
        for i in self.get_rlos() {
            let rlo_grade = i.get_rlo_grade();
            let rlo_weight = i.get_rlo_weight() * 0.01;

            grade += rlo_grade * rlo_weight;
        }
        grade
    }

    pub fn get_clo_weight(&self) -> f32 {
        self.weight_in_subject
    }

}

