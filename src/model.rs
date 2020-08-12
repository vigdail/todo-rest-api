use diesel::{PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl};

use super::schema::todos;

#[table_name = "todos"]
#[derive(Queryable, Insertable, Serialize, Clone, Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    pub completed: bool,
}

#[derive(Serialize, Insertable, Queryable, Deserialize, Clone, Debug)]
pub struct Todo {
    pub title: String,
    pub body: Option<String>,
}

impl From<Task> for Todo {
    fn from(todo: Task) -> Self {
        Todo {
            title: todo.title,
            body: todo.body,
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
        todos::dsl::todos.find(id).get_result(conn)
    }

    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Task>> {
        todos::dsl::todos.order(todos::id).load::<Task>(conn)
    }
}
