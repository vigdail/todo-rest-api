use diesel::{PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl};

use super::schema::todos;

#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    pub completed: bool,
}

#[derive(Serialize, Insertable, Queryable, Deserialize, AsChangeset)]
pub struct Todo {
    pub title: String,
    pub body: Option<String>,
    pub completed: Option<bool>,
}

impl From<Task> for Todo {
    fn from(todo: Task) -> Self {
        Todo {
            title: todo.title,
            body: todo.body,
            completed: Some(todo.completed),
        }
    }
}

impl Task {
    pub fn create(todo: Todo, conn: &PgConnection) -> QueryResult<Task> {
        diesel::insert_into(todos::table)
            .values(&todo)
            .get_result(conn)
    }

    pub fn get(id: i32, conn: &PgConnection) -> QueryResult<Task> {
        todos::table.find(id).get_result(conn)
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Task>> {
        todos::table.order(todos::id).load::<Task>(conn)
    }

    pub fn update(id: i32, todo: Todo, conn: &PgConnection) -> QueryResult<Task> {
        diesel::update(todos::table.find(id))
            .set(&todo)
            .get_result(conn)
    }

    pub fn delete(id: i32, conn: &PgConnection) -> QueryResult<Task> {
        diesel::delete(todos::table.find(id)).get_result(conn)
    }
}
