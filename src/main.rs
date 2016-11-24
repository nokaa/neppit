#![feature(proc_macro)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hayaku;
extern crate handlebars;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

mod board;
mod post;
mod thread;

use board::Board;

use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::rc::Rc;

use hayaku::{Http, Router, Request, ResponseWriter};
use handlebars::Handlebars;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    name: String,
    boards: HashMap<String, Board>,
    // boards: Vec<(String, String)>,
    rules: Vec<String>,
    port: Option<String>,
    proxy_ip_header: Option<String>,
}

impl Config {
    /*fn get_board_longname(&self, short_name: &String) -> Option<String> {
        for &(ref s, ref l) in &self.boards {
            if s == short_name {
                return Some(l.clone());
            }
        }
        None
    }*/
    fn get_board(&self, short_name: &String) -> Option<&Board> {
        self.boards.get(short_name)
    }
}

#[derive(Clone)]
struct Context {
    config: Config,
    templates: Rc<Handlebars>,
}

fn main() {
    env_logger::init().unwrap();
    info!("Starting up");

    let mut file = fs::File::open("config.toml").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let config: Config = toml::decode_str(&buf).unwrap();
    info!("{:?}", config);

    let addr = match config.port.clone() {
        Some(p) => {
            let addr = String::from("0.0.0.0") + &p;
            addr.parse().unwrap()
        }
        None => "0.0.0.0:3000".parse().unwrap(),
    };
    info!("listening on {}", addr);

    let mut templates = Handlebars::new();
    templates.register_template_file("home", "templates/home.hbs").unwrap();
    templates.register_template_file("board", "templates/board.hbs").unwrap();
    templates.register_template_file("thread", "templates/thread.hbs").unwrap();

    let ctx = Context {
        config: config,
        templates: Rc::new(templates),
    };

    let mut router = Router::new();
    router.get("/", Rc::new(home_handler)).unwrap();
    router.get("/b/:board", Rc::new(board_handler)).unwrap();
    router.get("/b/:board/:thread", Rc::new(thread_handler)).unwrap();

    let http = Http::new(router, ctx);
    http.listen_and_serve(addr);
}

fn home_handler(_req: &Request, res: &mut ResponseWriter, ctx: &Context) {
    let ref tmpl_ctx = ctx.config;
    let result = ctx.templates.render("home", &tmpl_ctx).unwrap();
    debug!("{}", result);
    res.write_all(result.as_bytes()).unwrap();
}

fn board_handler(req: &Request, res: &mut ResponseWriter, ctx: &Context) {
    let params = hayaku::get_path_params(req);
    let board = params.get("board").unwrap();
    let board = ctx.config.get_board(board).unwrap();

    let result = ctx.templates.render("board", &board).unwrap();
    debug!("{}", result);
    res.write_all(result.as_bytes()).unwrap();
}

fn thread_handler(req: &Request, res: &mut ResponseWriter, ctx: &Context) {
    let params = hayaku::get_path_params(req);
    let board = params.get("board").unwrap();
    let thread = params.get("thread").unwrap();
    let result = ctx.templates.render("thread", &()).unwrap();
    debug!("{}", result);
    res.write_all(result.as_bytes()).unwrap();
}
