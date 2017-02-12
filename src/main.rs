#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

mod db;
mod routes;

use routes::*;

fn main() {
    rocket::ignite()
        .mount("/",
               routes![home, board, create_thread, thread, create_post])
        .catch(errors![not_found])
        .launch()
}
