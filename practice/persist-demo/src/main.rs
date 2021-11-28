use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Gender {
    Unspecified,
    Male,
    Female,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub name: String,
    age: u8,
    gender: Gender,
}

impl User {
    pub fn new(name: String, age: u8, gender: Gender) -> Self {
        Self { name, age, gender }
    }

    pub fn persist(&self, filename: &str) -> Result<usize, std::io::Error> {
        let mut file = File::create(filename)?;
        let data = serde_json::to_string(self)?;
        file.write_all(data.as_bytes())?;
        Ok(data.len())
    }

    pub fn load(filename: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(filename)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let user = serde_json::from_str(&data)?;
        Ok(user)
    }
}

impl Default for User {
    fn default() -> Self {
        User::new("Unknown User".into(), 0, Gender::Unspecified)
    }
}

fn main() {
    let user = User::default();
    user.persist("/tmp/user1").unwrap();
    let user1 = User::load("/tmp/user1").unwrap();
    assert_eq!(user, user1);
}
