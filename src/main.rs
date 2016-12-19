/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 *
 * Ἥφαιστος
 * */
#![feature(proc_macro)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate hayaku;
extern crate handlebars;
extern crate dotenv;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

mod board;
mod database;
mod errors;
mod post;
mod routes;

use board::NewBoard;
use database as db;
use routes::*;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Read;
use std::sync::Arc;

use dotenv::dotenv;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use hayaku::{Http, Router};
use handlebars::Handlebars;

lazy_static! {
    static ref EMPTY_STRING: String = String::from("");
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    name: String,
    boards: HashMap<String, NewBoard>,
    rules: Vec<String>,
    port: Option<String>,
    proxy_ip_header: Option<String>,
}

#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub templates: Arc<Handlebars>,
    pub db_pool: db::Pool,
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

    // Read database url from `.env`
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    info!("db url: {}", database_url);

    // Create db connection pool
    let r2d2_config = r2d2::Config::default();
    let manager = PostgresConnectionManager::new(database_url, TlsMode::None).unwrap();
    let pool = r2d2::Pool::new(r2d2_config, manager).expect("Failed to create pool");

    // Create the tables if they do not already exist
    info!("Creating tables");
    db::create_tables(pool.clone()).unwrap();
    // Create new boards listed in config
    let boards: Vec<NewBoard> = config.boards.values().cloned().collect();
    info!("Creating boards {:?}", boards);
    db::create_boards(pool.clone(), &boards[..]).unwrap();

    let ctx = Context {
        config: config,
        templates: Arc::new(templates),
        db_pool: pool,
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

    let mut http = Http::new(router, ctx);
    http.sanitize();
    info!("listening on {}", addr);
    http.listen_and_serve(addr);
}
