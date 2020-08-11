table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        body -> Nullable<Varchar>,
        completed -> Bool,
    }
}
