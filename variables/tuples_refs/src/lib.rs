pub struct Student{
    pub age: u32,
    pub first_name: String,
    pub last_name: String,
}

pub fn id(student: &Student) -> u32 {
    return student.age;
}

pub fn first_name(student: &Student) -> &str {
    return &student.first_name;
}

pub fn last_name(student: &Student) -> &str {
    return &student.last_name;
}
