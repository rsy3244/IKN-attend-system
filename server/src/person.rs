use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct PersonRaw {
    id : usize,
    username : String,
    role : String,
    state : usize,
}

pub struct Person {
    id : usize,
    username : String,
    role : Role,
    state : State,
}

impl Person {
    pub fn new(id: usize, username: String, role: Role, state: State) -> Person {
        Person { id, username, role, state }
    }

    pub fn to_raw(&self) -> PersonRaw {
        PersonRaw {
            id: self.id,
            username: self.username.clone(),
            role: self.role.to_string(),
            state: self.state.to_usize(),
        }
    }
}

#[non_exhaustive]
#[allow(dead_code)]
pub enum State {
    Attend,
    Leave,
}

impl State {
    pub fn to_usize(&self) -> usize {
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

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::B1 => "B1".to_string(),
            Role::B2 => "B2".to_string(),
            Role::B3 => "B3".to_string(),
            Role::B4 => "B4".to_string(),
            Role::M1 => "M1".to_string(),
            Role::M2 => "M2".to_string(),
            Role::D1 => "D1".to_string(),
            Role::D2 => "D2".to_string(),
            Role::D3 => "D3".to_string(),
            Role::Professor => "Professor".to_string(),
            Role::AssociateProfessor => "AssociateProfessor".to_string(),
            Role::ResearchAssociate => "ResearchAssociate".to_string(),
            _ => panic!("not implemented!")
        }
    }
}
