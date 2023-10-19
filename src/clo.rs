use super::subject::difficulty_type;
use super::rlo::RLO;

#[derive(Debug, Clone)]
pub struct CLO {
    name: f32,
    difficulty_type: difficulty_type,
    RLOs: Vec<RLO>,
}

impl CLO {
    pub fn new(name: f32, difficulty_type: difficulty_type) -> Self {
        return CLO { name, difficulty_type, RLOs: Vec::new() }
    }

    pub fn add_rlo(&mut self, name: f32, weight_in_clo: f32) {
        let rlo_to_be_added = RLO::new(name, self.difficulty_type.clone(), weight_in_clo);
        self.RLOs.push(rlo_to_be_added);
    }

    pub fn get_rlos(&self) -> &Vec<RLO> {
        &self.RLOs
    }

    /// Simple weighted average of each child
    /// RLO
    pub fn get_clo_grade(&self) -> f32 {
        let mut grade = 0.0;
        for i in self.get_rlos() {
            let rlo_grade = i.get_rlo_grade();
            let rlo_weight = i.get_rlo_weight();

        }
        0.0
    }

}

