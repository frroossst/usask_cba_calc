
#[derive(Debug, Clone)]
enum DifficultyType {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone)]
pub struct CLO {
    pub name: f32,
//    difficulty_type: DifficultyType,
//    rlos: Vec<RLO>,
//    weight_in_subject: f32,
//    pub current_grade: Result<f32, f32>,
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
    #[cfg(feature = "segfault")]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }

    #[cfg(not(feature = "segfault"))]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clo() {
        let clo = CLO {
            name: 1.0,
//            difficulty_type: DifficultyType::Easy,
//            weight_in_subject: 1.0,
//            current_grade: Ok(1.0),
        };
        let clo2 = CLO {
            name: 2.0,
//            difficulty_type: DifficultyType::Easy,
//            weight_in_subject: 1.0,
//            current_grade: Ok(1.0),
        };
        let clo3 = CLO {
            name: 3.0,
//            difficulty_type: DifficultyType::Easy,
//            weight_in_subject: 1.0,
//            current_grade: Ok(1.0),
        };
        let clo4 = CLO {
            name: 4.0,
//            difficulty_type: DifficultyType::Easy,
//            weight_in_subject: 1.0,
//            current_grade: Ok(1.0),
        };

        let mut vec = vec![clo.clone(), clo2.clone(), clo3.clone(), clo4.clone()];
        vec.sort();
    }
}




fn main() {
    println!("Hello, world!");
}
