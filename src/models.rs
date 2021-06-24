use super::schema::todo;

#[derive(serde::Serialize, serde::Deserialize, Queryable)]
pub struct Todo{
    pub id: i32,
    pub task:String,
    pub user:String,
    pub iscompleted:String,
}

#[derive(Insertable,serde::Deserialize)]
#[table_name="todo"]
pub struct Addable{
    pub task:String,
    pub user:String,
    iscompleted:String,
}

