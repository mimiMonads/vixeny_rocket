
#[macro_use] 
extern crate rocket;

use rocket::Request;

#[get("/")]
fn index() -> &'static str {
    "Hello world"
}

#[get("/get/<id>")]
fn get_id(id: u32) -> String {
    format!("Received id: {}", id)
}

#[get("/a/b/c/d/e/f")]
fn nested() -> &'static str {
    "You've reached /a/b/c/d/e/f!"
}

#[get("/query?<q>")]
fn query(q: String) -> String {
    format!("Received query q={}", q)
}

#[get("/get/<name>/<id>/<num>")]
fn multi_param(name: String, id: u32, num: u32) -> String {
    format!("Name: {}, ID: {}, Num: {}", name, id, num)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, get_id,multi_param, nested, query])
        .register("/", catchers![not_found])
}
