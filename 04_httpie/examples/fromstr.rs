use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl FromStr for Person {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();

        if parts.len() != 2 {
            return Err("Invalid input format. Expected 'name,age'.");
        }

        let name: String = parts[0].trim().to_string();

        let age: u32 = match parts[1].trim().parse::<u32>() {
            Ok(age) => age,
            Err(_) => return Err("Invalid age. Must be a number."),
        };

        Ok(Person { name, age })
    }
}

fn main() {
    let person_str: &str = "Alice,12";
    let person: Person = Person::from_str(person_str).unwrap();

    println!("{:?}", person);
}
