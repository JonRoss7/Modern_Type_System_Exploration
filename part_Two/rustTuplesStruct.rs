struct Student {
    id: i32,
    name: String,
}

fn main() {
    let tuple_student: (i32, String) = (1, String::from("Alice"));
    println!("Tuple Student -> ID: {}, Name: {}", tuple_student.0, tuple_student.1);

    let record_student = Student { 
        id: 1, 
        name: String::from("Alice") 
    };
    println!("Record Student -> ID: {}, Name: {}", record_student.id, record_student.name);
}