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
        self.students_by_grade.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.students_by_grade.get(&grade) {
            Some(students) => students.iter().map(|s| s.to_string()).collect(),
            None => vec![],
        }
    }
}
