use super::clo::CLO;

#[derive(Debug, Clone)]
pub enum difficulty_type {
    TypeA,
    TypeB,
    TypeBPlus,
    TypeC,
}

/// Type A = no pass requirement
/// Type B = 70% minimum
/// Type B+ = 50% minimum
/// Type C = no pass requirement
pub fn pass_fall_check_per_difficulty(grade: u32, difficulty_type: difficulty_type) -> bool {
    match difficulty_type {
        difficulty_type::TypeA => true,
        difficulty_type::TypeB => grade >= 70,
        difficulty_type::TypeBPlus => grade >= 50,
        difficulty_type::TypeC => true,
    }
}

pub struct subject {
    name: String,
    CLOs: Vec<CLO>,
}

impl subject {
    pub fn new(name: String) -> subject {
        subject { name, CLOs: Vec::new() }
    }

    pub fn add_clo(&mut self, name: f32, difficulty_type: difficulty_type) {
        let clo_to_be_added = CLO::new(name, difficulty_type);
        self.CLOs.push(clo_to_be_added);
    }

    pub fn get_clos(&self) -> &Vec<CLO> {
        &self.CLOs
    }
}

