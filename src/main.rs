/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 *
 * Ἥφαιστος
 * */
#![feature(proc_macro)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hayaku;
extern crate handlebars;
extern crate dotenv;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

mod board;
mod database;
mod schema;
mod post;
mod thread;

use board::NewBoard;
use database as db;

use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::sync::Arc;

use hayaku::{Http, Router, Request, Response, Status};
use handlebars::Handlebars;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    name: String,
    boards: HashMap<String, NewBoard>,
    rules: Vec<String>,
    port: Option<String>,
    proxy_ip_header: Option<String>,
}

impl Config {
    fn get_board(&self, short_name: &String) -> Option<&NewBoard> {
        self.boards.get(short_name)
    }
}

#[derive(Clone)]
struct Context {
    config: Config,
    templates: Arc<Handlebars>,
}

fn main() {
    env_logger::init().unwrap();
    info!("Starting up");

    // Read the config file and deserialize it to a `Config`.
    let mut file = fs::File::open("config.toml").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let config: Config = toml::decode_str(&buf).unwrap();
    info!("{:?}", config);

    // Get the server address
    let addr = match config.port.clone() {
        Some(p) => {
            let addr = String::from("0.0.0.0") + &p;
            addr.parse().unwrap()
        }
        None => "0.0.0.0:3000".parse().unwrap(),
    };

    // Register the templates we will use
    let mut templates = Handlebars::new();
    templates.register_template_file("home", "templates/home.hbs").unwrap();
    templates.register_template_file("board", "templates/board.hbs").unwrap();
    templates.register_template_file("thread", "templates/thread.hbs").unwrap();
    templates.register_template_file("404", "templates/404.hbs").unwrap();

    // Open a connection to Postgres
    let boards: Vec<NewBoard> = config.boards.values().map(|b| b.clone()).collect();
    let connection = db::establish_connection();
    db::create_boards(&connection, &boards);

    let ctx = Context {
        config: config,
        templates: Arc::new(templates),
    };

    let mut router = Router::new();
    router.get("/", Arc::new(home_handler)).unwrap();
    router.get("/404", Arc::new(not_found_handler)).unwrap();
    router.get("/b/{board:[:alnum:]+}", Arc::new(board_handler)).unwrap();
    router.post("/b/{board:[:alnum:]+}", Arc::new(new_thread_handler)).unwrap();
    router.get(r"/b/{board:[:alnum:]+}/{thread:[\d]+}",
             Arc::new(thread_handler))
        .unwrap();
    router.set_not_found_handler(Arc::new(not_found_handler));

    let http = Http::new(router, ctx).sanitize();
    info!("listening on {}", addr);
    http.listen_and_serve(addr);
}

fn home_handler(_req: &Request, res: &mut Response, ctx: &Context) {
    info!("home hander");
    let ref tmpl_ctx = ctx.config;
    let result = ctx.templates.render("home", &tmpl_ctx).unwrap();
    debug!("{}", result);
    res.body(result.as_bytes()).unwrap();
}

fn board_handler(req: &Request, res: &mut Response, ctx: &Context) {
    info!("board hander");
    let params = hayaku::get_path_params(req);
    let board = params.get("board").unwrap();
    let board = if let Some(b) = ctx.config.get_board(board) {
        b
    } else {
        return not_found_handler(req, res, ctx);
    };


    let result = ctx.templates.render("board", &board).unwrap();
    debug!("{}", result);
    res.body(result.as_bytes()).unwrap();
}

fn new_thread_handler(req: &Request, _res: &mut Response, _ctx: &Context) {
    info!("new thread hander");
    let params = hayaku::get_path_params(req);
    let board = params.get("board").unwrap();
    let name = req.form_value("name").unwrap();
    let subject = req.form_value("subject").unwrap();
    let email = req.form_value("email").unwrap();
    let content = req.form_value("content").unwrap();
    // Write to database
}

fn thread_handler(req: &Request, res: &mut Response, ctx: &Context) {
    info!("thread hander");
    let params = hayaku::get_path_params(req);
    let board = params.get("board").unwrap();
    // let board = if let Some(b) = ctx.config.get_board(board) {
    // b
    // } else {
    // return not_found_handler(req, res, ctx);
    // };

    let thread = params.get("thread").unwrap();
    // let thread = if let Some(t) = board.get_thread(thread) {
    // t
    // } else {
    // return not_found_handler(req, res, ctx);
    // };

    let result = ctx.templates.render("thread", &(board, thread)).unwrap();
    debug!("{}", result);
    res.body(result.as_bytes()).unwrap();
}

fn not_found_handler(_req: &Request, res: &mut Response, ctx: &Context) {
    info!("not found hander");
    let result = ctx.templates.render("404", &()).unwrap();
    debug!("{}", result);
    res.status(Status::NotFound);
    res.body(result.as_bytes()).unwrap();
}
