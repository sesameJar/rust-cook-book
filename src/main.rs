#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

fn main() {
    let mut people = vec![
        Person::new("John".to_string(), 10),
        Person::new("Pet".to_string(), 30),
        Person::new("Al".to_string(), 1),
    ];

    people.sort();
    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 1),
            Person::new("John".to_string(), 10),
            Person::new("Pet".to_string(), 30),
        ]
    );

    people.sort_by(|a, b| b.age.cmp(&a.age));
    assert_eq!(
        people,
        vec![
            Person::new("Pet".to_string(), 30),
            Person::new("John".to_string(), 10),
            Person::new("Al".to_string(), 1),
        ]
    );
}
