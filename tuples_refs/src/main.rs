use tuples_refs::*;

fn main() {
    let student = Student {
        age: 20,
        first_name: "Pedro".to_string(),
        last_name: "Domingos".to_string(),
    };
    println!("Student's first name: {}", first_name(&student));
    println!("Student's last name: {}", last_name(&student));
    println!("Student's id: {}", id(&student));
}
