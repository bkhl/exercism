use std::collections::{BTreeMap, BTreeSet};

pub struct School<'a> {
    grades: BTreeMap<u32, BTreeSet<&'a str>>,
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        School {
            grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        self.grades
            .entry(grade)
            .or_insert(BTreeSet::new())
            .insert(student);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().map(|x| *x).collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.grades.get(&grade) {
            Some(ref x) => Some(x.iter().map(|x| x.to_string()).collect()),
            None => None,
        }
    }
}