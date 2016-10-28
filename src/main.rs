#[macro_use]
extern crate log;
extern crate env_logger;
extern crate regex;
extern crate hayaku;

use std::rc::Rc;

use hayaku::{Http, Request, ResponseWriter, util};
use regex::Regex;

fn main() {
    env_logger::init().unwrap();
    info!("Starting up");
    let addr = "127.0.0.1:3000".parse().unwrap();

    let mut http = Http::new(());
    http.handle_func(Regex::new(r"/").unwrap(), Rc::new(new_paste));
    http.listen_and_serve(addr);
}

fn new_paste(_req: &Request, res: &mut ResponseWriter, _ctx: &()) {
    util::send_string(res, b"hello, world!");
}
