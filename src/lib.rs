#[macro_use]
extern crate rocket;

mod routes;

use rocket_dyn_templates::Template;

#[launch]
pub fn serve() -> _ {
    rocket::build()
        .mount("/", routes![routes::home::view])
        .attach(Template::fairing())
}
