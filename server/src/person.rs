use super::schema::people;
use diesel::backend::Backend;
use diesel::deserialize::Queryable;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Insertable, Deserialize)]
#[table_name = "people"]
pub struct NewPerson {
    pub username: String,
    pub role: String,
    pub roomid: Option<i32>,
}

#[derive(Clone, Deserialize, Serialize, Queryable)]
pub struct Person {
    id: i32,
    username: String,
    role: Role,
    state: State,
    roomid: Option<i32>,
}

impl Person {
    pub fn new(id: i32, username: String, role: Role, state: State, roomid: Option<i32>) -> Person {
        Person {
            id,
            username,
            role,
            state,
            roomid,
        }
    }
}

#[derive(Clone, Serialize_repr, Deserialize_repr)]
#[serde(tag = "state")]
#[repr(u8)]
#[allow(dead_code)]
#[non_exhaustive]
pub enum State {
    Leave = 0,
    Attend = 1,
}

#[allow(unreachable_patterns)]
impl State {
    pub fn to_i32(&self) -> i32 {
        use State::*;
        match self {
            Leave => 0,
            Attend => 1,
            _ => panic!("not implemented!"),
        }
    }
}

impl<DB, ST> Queryable<ST, DB> for State
where
    DB: Backend,
    i32: Queryable<ST, DB>,
{
    type Row = <i32 as Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> Self {
        match i32::build(row) {
            0 => State::Leave,
            1 => State::Attend,
            _ => panic!("not implemented!"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
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

impl<DB, ST> Queryable<ST, DB> for Role
where
    DB: Backend,
    String: Queryable<ST, DB>,
{
    type Row = <String as Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> Self {
        match String::build(row).as_str() {
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
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
