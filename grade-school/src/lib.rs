use std::collections::HashMap;

pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades.entry(grade).or_insert(Vec::new()).push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut vec = self.grades
            .iter()
            .map(|(g, _)| *g)
            .collect::<Vec<u32>>();
        vec.sort();
        vec
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        if self.grades.contains_key(&grade) {
            let mut vec = self.grades[&grade].clone();
            vec.sort();
            Some(vec)
        } else {
            None
        }
    }
}
