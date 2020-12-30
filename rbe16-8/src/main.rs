trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn com_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}.",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct S {
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}

impl Person for S {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Student for S {
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Programmer for S {
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl CompSciStudent for S {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

fn main() {
    let s = S {
        name: "xiaoming".to_string(),
        university: "qinghua".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "xiaoming".to_string(),
    };

    println!("{}", com_sci_student_greeting(&s));
}
