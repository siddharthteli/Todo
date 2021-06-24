#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
mod models;
mod schema;
//mod auth;
 

//use auth::BasicAuth;
use diesel::prelude::*;
use models::*;
use rocket::response::status;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket_sync_db_pools::database;
use schema::*;

#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);

#[get("/todo")]
async fn view_all_task(conn: DbConn) -> Value {
    conn.run(|c| {
        let all = todo::table
            .limit(100)
            .load::<Todo>(c)
            .expect("Error loading rustaceans from DB!");
        json!(all)
    })
    .await
}

#[get("/todo/<id>")]
async fn view_task(id: i32, conn: DbConn) -> Value {
    conn.run(move |c| {
        let rustacean = todo::table
            .find(id)
            .get_result::<Todo>(c)
            .expect("Error loading rustacean from DB");
        json!(rustacean)
    })
    .await
}

#[post("/todo", format = "json", data = "<todo>")]
async fn create_task(
    conn: DbConn,
    todo: Json<Todo>,
) -> Value {
    conn.run(|c| {
        let result = diesel::insert_into(todo::table)
            .values(todo.into_inner())
            .execute(c)
            .expect("Error adding rustaceans to DB");
        json!(result)
    })
    .await
} 


#[delete("/todo/<id>")]
async fn delete_task(
    conn:DbConn,
    id:i32,
) -> Value{
    conn.run(move |c| {
       let result= diesel::delete(todo::table.find(id))
        .execute(c)
        .expect("Can't delete or ID doesn't exist");
        json!(result)
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!({ "success": false, "data": "Not found!"})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                view_all_task,
                view_task,
                create_task,
                delete_task,


            ],
        )
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
}

