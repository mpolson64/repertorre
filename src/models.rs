use super::schema::{positions, moves};

#[derive(Queryable)]
pub struct Position {
    pub id: i32,
    pub fen: String,
}

#[derive(Insertable)]
#[table_name="positions"]
pub struct NewPosition<'a> {
    pub fen: &'a str,
}

#[derive(Queryable)]
pub struct Move {
    pub id: i32,
    pub san: String,
    pub from_id: i32,
    pub to_id: i32,
}

#[derive(Insertable)]
#[table_name="moves"]
pub struct NewMove<'a> {
    pub san: &'a str,
    pub from_id: &'a i32,
    pub to_id: &'a i32,
}