#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket_contrib::json::{Json};

use self::diesel::prelude::*;

pub mod schema;
pub mod models;
use self::models::*;
use schema::dog::dsl::*;
use schema::dog;

#[database("my_test_db")]
struct MyDbConn(diesel::PgConnection);


#[post("/", data = "<new_dog_json>")]
fn json_post<'a>(conn: MyDbConn, new_dog_json: Json<NewDog>) -> Json<Dog> {
    let new_dog = new_dog_json.into_inner();
    let result = diesel::insert_into(dog::table)
        .values(&new_dog)
        .get_result::<Dog>(&*conn)
        .expect("db error on insert");
    Json(result)
}

#[get("/")]
fn json_get<'a>(conn: MyDbConn) -> Json<Vec<Dog>> {
    let dogs = dog.load::<Dog>(&*conn).expect("db error on select");
    Json(dogs)
}

fn main() {
    rocket::ignite()
        .attach(MyDbConn::fairing())
        .mount("/", routes![json_post])
        .mount("/", routes![json_get])
        .launch();
}
