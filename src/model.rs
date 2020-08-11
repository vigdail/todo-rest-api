use diesel::{PgConnection, Queryable, RunQueryDsl};

use super::schema::todos;

#[table_name = "todos"]
#[derive(Queryable, Insertable, Serialize, Clone, Debug)]
pub struct Task {
    pub id: Option<i32>,
    pub title: String,
    pub body: Option<String>,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub body: Option<String>,
}

impl Task {
    pub fn create(todo: Todo, conn: &PgConnection) -> bool {
        let insert = Task {
            id: None,
            title: todo.title,
            body: todo.body,
            completed: false,
        };
        diesel::insert_into(todos::table)
            .values(&insert)
            .execute(conn)
            .is_ok()
    }
}
