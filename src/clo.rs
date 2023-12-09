use super::subject::DifficultyType;
use std::cmp::Ordering;
use super::rlo::RLO;

#[derive(Debug, Clone)]
pub struct CLO {
    pub name: f32,
    difficulty_type: DifficultyType,
    rlos: Vec<RLO>,
    weight_in_subject: f32,
    pub current_grade: Result<f32, f32>,
}

impl CLO {
    pub fn new(name: f32, difficulty_type: DifficultyType, weight_in_subject: f32) -> Self {
        CLO { name, difficulty_type, rlos: Vec::new(), weight_in_subject, current_grade: Err(f32::NAN) }
    }

    pub fn add_rlo(&mut self, name: f32, weight_in_clo: f32) {
        let rlo_to_be_added = RLO::new(name, self.difficulty_type.clone(), weight_in_clo);
        self.rlos.push(rlo_to_be_added);
        self.rlos.sort();
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
        self.current_grade = self.get_if_clo_pass(grade);
        grade
    }

    fn get_if_clo_pass(&self, grade: f32) -> Result<f32, f32> {
        match self.difficulty_type {
            DifficultyType::TypeB => {
                if grade < 75.0 {
                    Err(grade)
                }
                else {
                    Ok(grade)
                }
            }
            DifficultyType::TypeBPlus => {
                if grade < 50.0 {
                    Err(grade)
                }
                else {
                    Ok(grade)
                }
            }
            _ => Ok(grade)
        }
    }

    pub fn get_clo_weight(&self) -> f32 {
        self.weight_in_subject
    }
}

impl Ord for CLO {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for CLO {}

impl PartialEq for CLO {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }

}

impl PartialOrd for CLO {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // self.name.partial_cmp(&other.name)
        // ! causes segfault
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod clo_tests {

    use super::*;

    #[test]
    fn test_clo_sort() {
        let clo0 = CLO::new(0.0, DifficultyType::TypeA, 0.0);
        let clo1 = CLO::new(1.0, DifficultyType::TypeA, 0.0);
        let clo2 = CLO::new(2.0, DifficultyType::TypeA, 0.0);
        let clo3 = CLO::new(3.0, DifficultyType::TypeA, 0.0);
        let clo4 = CLO::new(4.0, DifficultyType::TypeA, 0.0);

        let mut clo_vec = vec![&clo4, &clo2, &clo3, &clo1, &clo0];

        clo_vec.sort();

        assert_eq!(clo_vec[0].name, 0.0);
        assert_eq!(clo_vec[1].name, 1.0);
        assert_eq!(clo_vec[2].name, 2.0);
        assert_eq!(clo_vec[3].name, 3.0);
        assert_eq!(clo_vec[4].name, 4.0);
    }

}
