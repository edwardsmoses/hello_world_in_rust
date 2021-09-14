#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: i32,
    pub hobby: String,
    pub job_title: String,
}

fn main() {
    create_variables_and_run();

    println!("Hello, world!");
}

fn create_variables_and_run() {
    let none = "Name";
    let number_okay: i32 = 32;
    let name: &str = "Name of place";

    println!("{}", none);
    println!("{}", number_okay);
    println!("{}", name);

    let me: Person = Person {
        name: "Edwards Moses".to_string(),
        age: 22,
        hobby: "Coding Rust".to_string(),
        job_title: "Software Developer".to_string(),
    };
    println!("{:#?}", me);
}
