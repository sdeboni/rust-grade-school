use std::collections::BTreeMap;
use std::collections::BTreeSet;

pub struct School {
    students_by_grade: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students_by_grade: BTreeMap::new(),
        }
    }

    fn is_student_in_any_grade(&self, name: &str) -> bool {
        for (_grade, students) in self.students_by_grade.iter() {
            if students.contains(name) {
                return true;
            }
        }
        false
    }

    pub fn add(&mut self, grade: u32, name: &str) {
        if self.is_student_in_any_grade(name) {
            return;
        }

        self.students_by_grade
            .entry(grade)
            .or_default()
            .insert(name.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students_by_grade.keys().copied().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.students_by_grade.get(&grade) {
            Some(students) => students.iter().cloned().collect(),
            None => vec![],
        }
    }
}
