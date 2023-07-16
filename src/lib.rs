#[macro_use]
extern crate rocket;

#[launch]
pub fn serve() -> _ {
    rocket::build()
}
