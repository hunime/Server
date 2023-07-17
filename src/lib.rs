#[macro_use]
extern crate rocket;

pub mod utils;

mod routes;

use rocket_dyn_templates::Template;

#[launch]
pub fn serve() -> _ {
    rocket::build()
        .mount("/", routes![routes::home::view])
        .attach(Template::fairing())
}
