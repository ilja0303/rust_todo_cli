use diesel::table;
table! {
    todos (id) {
        id -> Int4,
        todo_title -> Varchar,
    }
}
