trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer 
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {
    // Example usage
    struct StudentImpl;

    impl Person for StudentImpl {
        fn name(&self) -> String {
            "Alice".into()
        }
    }

    impl Student for StudentImpl {
        fn university(&self) -> String {
            "Wonderland University".into()
        }
    }

    impl Programmer for StudentImpl {
        fn fav_language(&self) -> String {
            "Rust".into()
        }
    }

    impl CompSciStudent for StudentImpl {
        fn git_username(&self) -> String {
            "alice_wonderland".into()
        }
    }

    let student = StudentImpl;
    println!("{}", comp_sci_student_greeting(&student));
}