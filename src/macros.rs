#[macro_export]

/// takes in two inputs
/// 1. current grade
/// 2. recent grade
/// returns the expression for the weighted grade
/// This is to ensure compatibility with the weighted grade as they change
macro_rules! weighted_rlo_grade {
    ($current_grade:expr, $recent_grade:expr) => {
        ($current_grade * 0.6) + ($recent_grade * 0.4)
    };
}
