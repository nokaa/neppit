/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 * */
pub mod types;

use {Context, db};
use errors::*;
use self::types::*;

use rocket::State;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Template;

#[get("/")]
pub fn home(ctx: State<Context>) -> Template {
    Template::render("home", &ctx.config)
}

#[get("/b/<board_name>")]
pub fn board(ctx: State<Context>, board_name: &str) -> Result<Template> {
    let board = ctx.config.boards.get(board_name);
    if board.is_none() {
        return Ok(not_found());
    }
    let pool = ctx.db_pool.clone();
    let mut catalog = db::read::catalog(pool, board_name)?;
    catalog.reverse();
    Ok(Template::render("board", &(board.unwrap(), catalog)))
}

#[post("/b/<board>", data = "<new_thread_form>")]
pub fn create_thread(ctx: State<Context>,
                     board: &str,
                     new_thread_form: Form<NewThread>)
                     -> Result<Option<Redirect>> {
    if ctx.config.boards.get(board).is_none() {
        return Ok(None);
    }

    let new_thread_form = new_thread_form.get();
    if new_thread_form.content.is_empty() && new_thread_form.subject.is_empty() {
        return Ok(None);
    }

    let pool = ctx.db_pool.clone();
    let thread_number = db::create::thread(pool, board, new_thread_form)?;
    let redirect = format!("/b/{}/{}", board, thread_number);
    Ok(Some(Redirect::to(&redirect)))
}

#[get("/b/<board_name>/<thread>")]
pub fn thread(ctx: State<Context>, board_name: &str, thread: i64) -> Result<Template> {
    let board = ctx.config.boards.get(board_name);
    if board.is_none() {
        return Ok(not_found());
    }

    let pool = ctx.db_pool.clone();
    let thread = db::read::thread(pool, board_name, thread)?;
    Ok(Template::render("thread", &(board.unwrap(), thread)))
}

#[post("/b/<board>/<thread>", data = "<new_post_form>")]
pub fn create_post(ctx: State<Context>,
                   board: &str,
                   thread: i64,
                   new_post_form: Form<NewPost>)
                   -> Result<Option<Redirect>> {
    if ctx.config.boards.get(board).is_none() {
        return Ok(None);
    }

    let new_post_form = new_post_form.get();
    if new_post_form.content.is_empty() {
        return Ok(None);
    }

    let pool = ctx.db_pool.clone();
    let post_number = db::create::post(pool, board, thread, new_post_form)?;

    let thread = format!("/b/{}/{}#{}", board, thread, post_number);
    Ok(Some(Redirect::to(&thread)))
}

#[error(404)]
pub fn not_found() -> Template {
    Template::render("404", &())
}
