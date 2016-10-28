#![feature(proc_macro)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate regex;
extern crate hayaku;
extern crate handlebars;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

use std::fs;
use std::io::Read;
use std::rc::Rc;

use hayaku::{Http, Request, ResponseWriter, util};
use handlebars::Handlebars;
use regex::Regex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    name: String,
    boards: Vec<String>,
    rules: Vec<String>,
    port: Option<String>,
    proxy_ip_header: Option<String>,
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
    file.read_to_string(&mut buf);

    let config: Config = toml::decode_str(&buf).unwrap();
    info!("{:?}", config);

    let addr = match config.port.clone() {
        Some(p) => {
            let addr = String::from("0.0.0.0") + &p;
            //addr.add
            addr.parse().unwrap()
        }
        None => {
            "0.0.0.0:3000".parse().unwrap()
        }
    };
    info!("listening on {}", addr);

    let mut templates = Handlebars::new();
    templates.register_template_file("home", "templates/home.hbs").unwrap();

    let ctx = Context{
        config: config,
        templates: Rc::new(templates),
    };

    let mut http = Http::new(ctx);
    http.handle_func(Regex::new("/").unwrap(), Rc::new(home_handler));
    http.listen_and_serve(addr);
}

fn home_handler(_req: &Request, res: &mut ResponseWriter, ctx: &Context) {
    let ref tmpl_ctx = ctx.config;
    let result = ctx.templates.render("home", &tmpl_ctx).unwrap();
    info!("{}", result);
    util::send_string(res, result.as_bytes());
}
