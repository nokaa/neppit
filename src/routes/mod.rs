mod types;

use self::types::*;

use rocket::request::Form;
use rocket_contrib::Template;

#[get("/")]
pub fn home() -> Template {
    Template::render("home", &())
}

#[get("/<board>")]
pub fn board(board: &str) -> Template {
    Template::render("board", &())
}

#[post("/<board>", data = "<new_thread_form>")]
pub fn create_thread(board: &str, new_thread_form: Form<NewThread>) {}

#[get("/<board>/<thread>")]
pub fn thread(board: &str, thread: usize) -> Template {
    Template::render("thread", &())
}

#[post("/<board>/<thread>", data = "<new_post_form>")]
pub fn create_post(board: &str, thread: usize, new_post_form: Form<NewPost>) {}

#[error(404)]
pub fn not_found() -> Template {
    Template::render("404", &())
}
