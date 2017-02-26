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
use rocket::response::{NamedFile, Redirect};
use rocket_contrib::Template;

use std::path::{Path, PathBuf};

#[get("/")]
pub fn home(ctx: State<Context>) -> Template {
    Template::render("home", &ctx.config)
}

#[get("/resources/<file..>")]
pub fn resources(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("resources/").join(file)).ok()
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
    Ok(Template::render("board", &(&ctx.config, board.unwrap(), catalog)))
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

    let content = parse_content(&new_thread_form.content);
    let new_thread_form = NewThread {
        subject: new_thread_form.subject.clone(),
        content: content,
    };

    let pool = ctx.db_pool.clone();
    let thread_number = db::create::thread(pool, board, &new_thread_form)?;
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
    Ok(Template::render("thread", &(&ctx.config, board.unwrap(), thread)))
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
    let content = parse_content(&new_post_form.content);
    let new_post_form = NewPost { content: content };

    let pool = ctx.db_pool.clone();
    let post_number = db::create::post(pool, board, thread, &new_post_form)?;

    let thread = format!("/b/{}/{}#{}", board, thread, post_number);
    Ok(Some(Redirect::to(&thread)))
}

#[error(404)]
pub fn not_found() -> Template {
    Template::render("404", &())
}

fn parse_content(content: &str) -> String {
    let mut parsed = String::new();

    for line in content.lines() {
        if line.starts_with('>') {
            // green text
            let line = html_escape(line);
            parsed.push_str("<span class=\"gtext\">");
            parsed.push_str(&line);
            parsed.push_str("</span>")
        } else {
            let line = html_escape(line);
            parsed.push_str(&line);
        }
        parsed.push_str("<br>");
    }

    parsed
}

// Taken from handlebars-rust
use regex::{Captures, Regex};

lazy_static!{
    static ref DEFAULT_REPLACE: Regex = Regex::new(">|<|\"|&").unwrap();
}

fn html_escape(data: &str) -> String {
    DEFAULT_REPLACE.replace_all(data, |cap: &Captures| {
                       match cap.get(0).map(|m| m.as_str()) {
                           Some("<") => "&lt;",
                           Some(">") => "&gt;",
                           Some("\"") => "&quot;",
                           Some("&") => "&amp;",
                           _ => unreachable!(),
                       }
                       .to_owned()
                   })
                   .into_owned()
}
