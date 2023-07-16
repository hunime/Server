use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn view() -> Template {
    Template::render("Home", context! {})
}
