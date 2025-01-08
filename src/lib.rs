mod another_lib;

pub fn outsider() {
    another_lib::another_module::another_fn();
    println!("outsider function!");
}

pub mod learning_rust {

    pub trait Log {
        fn display_information(&self);
    }
    use std::fmt;
    pub enum PersonId {
        Passport(usize),
        IdentityCard(usize, usize, usize),
    }
    impl fmt::Display for PersonId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                PersonId::Passport(x) => {
                    write!(f, "{}", x)
                }
                PersonId::IdentityCard(x, y, z) => {
                    write!(f, "{} {} {}", x, y, z)
                }
            }
        }
    }

    pub struct Animal(pub String);
    impl Log for Animal {
        fn display_information(&self) {
            println!("{}", self.0);
        }
    }
    pub struct Person {
        name: String,
        last_name: String,
        age: usize,
        pub id: PersonId,
    }

    impl Person {
        pub fn new() -> Person {
            Person {
                name: "Default".to_string(),
                last_name: "Default".to_string(),
                age: 0,
                id: PersonId::IdentityCard(50, 40, 201),
            }
        }
        pub fn from(name: String, last_name: String, age: usize, id: PersonId) -> Person {
            Person {
                name,
                last_name,
                age,
                id,
            }
        }

        pub fn change_age(&mut self, new_age: usize) {
            self.age = new_age;
        }
        pub fn display_infor(&self) {
            println!("{} {} {} {}", self.name, self.last_name, self.age, self.id);
        }
    }
    impl Log for Person {
        fn display_information(&self) {
            println!("{} {} {} {}", self.name, self.last_name, self.age, self.id);
        }
    }
}
