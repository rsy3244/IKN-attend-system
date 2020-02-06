use serde::{Deserialize, Serialize};
use super::schema::people;

#[derive(Clone, Deserialize, Serialize, Queryable)]
pub struct PersonRaw {
    pub id: i32,
    pub username: String,
    pub role: String,
    pub state: i32,
}

impl PersonRaw {
    fn to_person(&self) -> Person {
        Person {
            id: self.id,
            username: self.username.clone(),
            role: match self.role.as_str() {
                "B1" => Role::B1,
                "B2" => Role::B2,
                "B3" => Role::B3,
                "B4" => Role::B4,
                "M1" => Role::M1,
                "M2" => Role::M2,
                "D1" => Role::D1,
                "D2" => Role::D2,
                "D3" => Role::D3,
                "Secretary" => Role::Secretary,
                "Professor" => Role::Professor,
                "AssociateProfessor" => Role::AssociateProfessor,
                "ResearchAssociate" => Role::ResearchAssociate,
                _ => panic!("not implemented!"),
            },
            state: match self.state {
                0 => State::Leave,
                1 => State::Attend,
                _ => panic!("not implemented!"),
            },
        }
    }
}

#[derive(Insertable, Deserialize)]
#[table_name="people"]
pub struct NewPerson<'a> {
    pub username: &'a str,
    pub role: &'a str,
}

pub struct Person {
    id: i32,
    username: String,
    role: Role,
    state: State,
}

impl Person {
    pub fn new(id: i32, username: String, role: Role, state: State) -> Person {
        Person { id, username, role, state }
    }

    pub fn to_raw(&self) -> PersonRaw {
        PersonRaw {
            id: self.id,
            username: self.username.clone(),
            role: self.role.to_string(),
            state: self.state.to_i32(),
        }
    }
}

#[allow(dead_code)]
#[non_exhaustive]
pub enum State {
    Attend,
    Leave,
}

#[allow(unreachable_patterns)]
impl State {
    pub fn to_i32(&self) -> i32 {
        use State::*;
        match self {
            Attend => 1,
            Leave => 0,
            _ => panic!("not implemented!")
        }
    }
}

#[non_exhaustive]
#[allow(dead_code)]
pub enum Role {
    B1,
    B2,
    B3,
    B4,
    M1,
    M2,
    D1,
    D2,
    D3,
    Secretary,
    Professor,
    AssociateProfessor,
    ResearchAssociate,
}

//#[allow(unreachable_patterns)]
//impl ToString for Role {
//    fn to_string(&self) -> String {
//        match self {
//            Role::B1 => "B1".to_string(),
//            Role::B2 => "B2".to_string(),
//            Role::B3 => "B3".to_string(),
//            Role::B4 => "B4".to_string(),
//            Role::M1 => "M1".to_string(),
//            Role::M2 => "M2".to_string(),
//            Role::D1 => "D1".to_string(),
//            Role::D2 => "D2".to_string(),
//            Role::D3 => "D3".to_string(),
//            Role::Professor => "Professor".to_string(),
//            Role::AssociateProfessor => "AssociateProfessor".to_string(),
//            Role::ResearchAssociate => "ResearchAssociate".to_string(),
//            _ => panic!("not implemented!")
//        }
//    }
//}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
