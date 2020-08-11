use diesel::Queryable;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    pub completed: bool,
}
