use std::collections::{BTreeMap, HashSet};

pub struct School {
    student_list:HashSet<String>,
    grades:BTreeMap<u32, HashSet<String>>
}

impl School {
    pub fn new() -> School {
        Self { 
            student_list:HashSet::new(),
            grades: BTreeMap::new()
         }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.student_list.contains(student) {
            return
        }
        self.student_list.insert(student.to_string());
        self.grades.entry(grade).or_default().insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
         self.grades.keys().cloned().collect()   
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut output:Vec<String> = Vec::new();
        if let Some(grades) = self.grades.get(&grade) {
            output.extend(grades.to_owned());
            output.sort();
        }
        output
    }
}
