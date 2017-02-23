/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 * */
#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
#[macro_use(log, info)]
extern crate log;
extern crate postgres;
extern crate rocket;
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_postgres;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

mod db;
mod errors;
mod routes;

use routes::*;

use dotenv::dotenv;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

use std::collections::HashMap;
use std::{env, fs};
use std::io::Read;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewBoard {
    short_name: String,
    long_name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    name: String,
    boards: HashMap<String, NewBoard>,
    rules: Vec<String>,
}

#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub db_pool: db::Pool,
}

fn main() {
    env_logger::init().unwrap();
    info!("Starting up");

    // Read the config file and deserialize it to a `Config`.
    let mut file = fs::File::open("neppit.toml").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let config: Config = toml::from_str(&buf).unwrap();
    info!("{:?}", config);

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
    db::create::tables(pool.clone()).expect("failed to create tables");
    // Create new boards listed in config
    let boards: Vec<NewBoard> = config.boards.values().cloned().collect();
    info!("Creating boards {:?}", boards);
    db::create::boards(pool.clone(), &boards[..]).unwrap();

    let ctx = Context {
        config: config,
        db_pool: pool,
    };

    rocket::ignite()
        .manage(ctx)
        .mount("/",
               routes![home, board, create_thread, thread, create_post])
        .catch(errors![not_found])
        .launch()
}
