use super::schema::todo;

#[derive(Queryable,Insertable,serde::Deserialize,serde::Serialize)]
#[table_name="todo"]
pub struct Todo{
    pub ID: i32,
    pub task:String,

}

