use std::fmt::{Display, Error, Formatter};
#[derive(Clone)]
pub struct Student {
    name: String,
    gpa: f64,
    year: i32,
}

impl Student {
    pub fn new(name: String, gpa: f64, year: i32) -> Self {
        Self { name, gpa, year }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_gpa(&self) -> f64 {
        self.gpa
    }

    pub fn get_year(&self) -> i32 {
        self.year
    }
}

impl Display for Student {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{{\n\tname: {},\n\tgpa: {}, \n\tyear: {}\n}}",
            self.name, self.gpa, self.year
        )
    }
}

pub struct StudentStatsArray<const N: usize> {
    students: [Student; N],
}
impl<const N: usize> StudentStatsArray<N> {
    pub fn new(students: [Student; N]) -> Self {
        Self { students }
    }

    pub fn average_gpa(&self) -> f64 {
        let mut avg = 0.0;
        for student in &self.students {
            avg += student.get_gpa();
        }

        avg / (self.students.len() as f64)
    }

    pub fn get_gpa_range(&self) -> f64 {
        let mut hi = -f64::INFINITY;
        let mut lo = f64::INFINITY;

        for k in &self.students {
            let gpa = k.get_gpa();
            if gpa > hi {
                hi = gpa;
            }

            if gpa < lo {
                lo = gpa;
            }
        }

        hi - lo
    }
    pub fn get_longest_name(&self) -> String {
        let mut longest = String::new();
        for student in &self.students {
            if student.get_name().len() > longest.len() {
                longest = student.get_name().clone();
            }
        }

        longest
    }

    pub fn get_num_freshman(&self) -> usize {
        let mut count = 0;
        for student in &self.students {
            if student.get_year() == 1 {
                count += 1;
            }
        }

        count
    }

    pub fn search_name(&self, name: String) -> i32 {
        for i in 0..self.students.len() {
            if self.students[i].get_name() == &name {
                return i as i32;
            }
        }

        -1
    }

    pub fn search_gpa(&self, gpa: f64) -> i32 {
        for i in 0..self.students.len() {
            if self.students[i].get_gpa() >= gpa {
                return i as i32;
            }
        }

        -1
    }

    pub fn sort_status(&self) -> SortStatus {
        if self.is_sorted() {
            SortStatus::Ascending
        } else if self.is_sorted_desc() {
            SortStatus::Descending
        } else {
            SortStatus::Unsorted
        }
    }

    fn is_sorted(&self) -> bool {
        if &self.students.len() == &1 || &self.students.len() == &0 {
            return true;
        }

        for i in 1..self.students.len() {
            if self.students[i].get_gpa() < self.students[i - 1].get_gpa() {
                return false;
            }
        }

        return true;
    }

    fn is_sorted_desc(&self) -> bool {
        let ck = self.students.clone();
        if ck.len() == 1 || ck.len() == 0 {
            return true;
        }

        for i in 1..ck.len() {
            if ck[i].get_gpa() > ck[i - 1].get_gpa() {
                return false;
            }
        }

        return true;
    }
}

pub enum SortStatus {
    Ascending,
    Descending,
    Unsorted,
}

impl<const N: usize> Display for StudentStatsArray<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[").unwrap();
        for student in &self.students {
            write!(
                f,
                "{{\n\tname: {},\n\tgpa: {},\n\tyear: {}\n}},",
                student.get_name(),
                student.get_gpa(),
                student.get_year()
            ).unwrap();
        }

        write!(f, "]")
    }
}
