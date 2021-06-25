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
//use rocket::response::status;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket_sync_db_pools::database;
use schema::*;

#[database("sqlite_db")]
struct DbConn(diesel::SqliteConnection);
// create new todo
#[get("/view-all")]
async fn view_all_todo(conn: DbConn) -> Value {
    conn.run(|c| {
        let result = todo::table
            .limit(100)
            .load::<Todo>(c)
            .expect("Error loading todo from DB!");
        json!({ "success": true, "data": result})
    })
    .await
}
//
#[get("/view/<id>")]
async fn view_one_todo(id: i32, conn: DbConn) -> Value {
    conn.run(move |c| {
        let result = todo::table
            .find(id)
            .get_result::<Todo>(c)
            .expect("Task doesn't task");
        json!({ "success": true, "data": result})
    })
    .await
}

#[post("/create", format = "json", data = "<update_todo>")]
async fn create_todo(conn: DbConn, update_todo: Json<InsertableTodo>) -> Value {
    conn.run(|c| {
        let result = diesel::insert_into(todo::table)
            .values(update_todo.into_inner())
            .execute(c)
            .expect("Error adding new task to DB");
        json!({ "success": true, "data": result})
    })
    .await
}

#[put("/update/<id>", format = "json", data = "<update_todo>")]
async fn update_todo(id: i32, conn: DbConn, update_todo: Json<Todo>) -> Value {
    conn.run(move |c| {
        let result = diesel::update(todo::table.find(id))
            .set((todo::iscompleted.eq(update_todo.iscompleted.to_owned()),
            todo::user.eq(update_todo.user.to_owned()),
            todo::task.eq(update_todo.task.to_owned())
             ))
            .execute(c)
            .expect("Error updating todo to DB");
        json!({ "success": true, "data": result})
    })
    .await
}

#[delete("/delete/<id>")]
async fn delete_todo(conn: DbConn, id: i32) -> Value {
    conn.run(move |c| {
        let result = diesel::delete(todo::table.find(id))
            .execute(c)
            .expect("Can't delete orcoz ID doesn't exist");
        json!({ "success": true, "data": result})
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
                view_all_todo,
                view_one_todo,
                create_todo,
                update_todo,
                delete_todo,
            ],
        )
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
}
