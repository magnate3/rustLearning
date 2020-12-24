// An imaginary magical school has a new report card generation system written in rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5). However, the school also issues alphabetical grades
// (A+ -> F-) and needs to be able to print both types of report card!

// Make the necessary code changes to support alphabetical report cards, thereby making the second
// test pass.

// I AM DONE It must be again to learn about display.

use std::fmt::Display;


//pub struct ReportCard {
pub struct ReportCard<T: Display> {

    // pub grade: f32,
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

/*
https://users.rust-lang.org/t/how-to-implement-struct-with-displayable-attribute/43175
Like this impl<T: Display> ReportCard<T>{ pub fn print(&self) -> String { format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, &self.grade) } } The T on the impl introduces the generic parameter, and the T on ReportCard uses the generic parameter
impl ReportCard {
    pub fn print(&self) -> String {
        match &self.grade {
            5.1..=5.5 => format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, "A+".to_string()),
            5.0 => format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, "A".to_string()),
            4.0..5.0 => format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, "B".to_string()),
            3.0..4.0 => format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, "C".to_string()),
            0.0..3.0 => format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, "F-".to_string()),
            _ =>  format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, "not format"),
        }
        //format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, &self.grade)
    }
}
*/
impl<T: Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
