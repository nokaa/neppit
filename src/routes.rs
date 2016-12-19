use hayaku::{self, Request, Response, Status};

use super::{Context, EMPTY_STRING};
use database as db;
use post::Post;

pub fn home_handler(_req: &Request, res: &mut Response, ctx: &Context) {
    info!("home handler");
    let tmpl_ctx = &ctx.config;
    let result = ctx.templates.render("home", &tmpl_ctx).unwrap();
    debug!("{}", result);
    res.body(result.as_bytes()).unwrap();
}

pub fn board_handler(req: &Request, res: &mut Response, ctx: &Context) {
    info!("board handler");
    let params = hayaku::get_path_params(req);
    let board = params.get("board").unwrap_or(&EMPTY_STRING);
    let pool = ctx.db_pool.clone();
    let board = if let Ok(Some(b)) = db::get_board(pool, board) {
        b
    } else {
        return not_found_handler(req, res, ctx);
    };


    let result = ctx.templates.render("board", &board).unwrap();
    debug!("{}", result);
    res.body(result.as_bytes()).unwrap();
}

pub fn new_thread_handler(req: &Request, res: &mut Response, ctx: &Context) {
    info!("new thread handler");
    let params = hayaku::get_path_params(req);
    let board = params.get("board").unwrap_or(&EMPTY_STRING);
    let name = req.form_value("name").unwrap_or("".to_string());
    let subject = req.form_value("subject").unwrap_or("".to_string());
    let email = req.form_value("email").unwrap_or("".to_string());
    let content = req.form_value("content").unwrap_or("".to_string());

    let pool = &ctx.db_pool;
    // Make sure that board exists
    let board_exists = db::board_exists(pool.clone(), board);
    if board_exists.is_err() || !board_exists.unwrap() {
        return not_found_handler(req, res, ctx);
    }

    // Get post number
    let post_number = if let Ok(num) = db::get_post_number(pool.clone(), board) {
        num
    } else {
        return not_found_handler(req, res, ctx);
    };

    // Write to database
    let thread = Post {
        post_number: post_number,
        board: board.clone(),
        subject: Some(subject),
        name: name,
        email: email,
        content: content,
        thread: true,
        parent: None,
    };
    if db::create_thread(pool.clone(), thread).is_err() {
        return not_found_handler(req, res, ctx);
    }
}

pub fn thread_handler(req: &Request, res: &mut Response, ctx: &Context) {
    info!("thread handler");
    let params = hayaku::get_path_params(req);
    let board = params.get("board").unwrap_or(&EMPTY_STRING);
    // let board = if let Some(b) = ctx.config.get_board(board) {
    // b
    // } else {
    // return not_found_handler(req, res, ctx);
    // };

    let thread = params.get("thread").unwrap_or(&EMPTY_STRING);
    // let thread = if let Some(t) = board.get_thread(thread) {
    // t
    // } else {
    // return not_found_handler(req, res, ctx);
    // };

    let result = ctx.templates.render("thread", &(board, thread)).unwrap();
    debug!("{}", result);
    res.body(result.as_bytes()).unwrap();
}

pub fn not_found_handler(_req: &Request, res: &mut Response, ctx: &Context) {
    info!("not found hander");
    let result = ctx.templates.render("404", &()).unwrap();
    debug!("{}", result);
    res.status(Status::NotFound);
    res.body(result.as_bytes()).unwrap();
}
