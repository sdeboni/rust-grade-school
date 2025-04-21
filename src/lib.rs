use std::collections::HashMap;
use std::collections::HashSet;

pub struct School {
    students_by_grade: HashMap<u32, HashSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students_by_grade: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        for (_, students) in self.students_by_grade.iter() {
            if students.contains(student) {
                return;
            }
        }

        self.students_by_grade
            .entry(grade)
            .or_default()
            .insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.students_by_grade.keys().copied().collect();
        grades.sort();
        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut ret = match self.students_by_grade.get(&grade) {
            Some(students) => students.iter().map(|s| s.to_string()).collect(),
            None => vec![],
        };
        ret.sort();
        ret
    }
}
