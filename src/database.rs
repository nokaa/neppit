use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;

use super::board::{Board, NewBoard};
use super::post::Post;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_boards(pool: Pool, boards: &[NewBoard]) {
    use schema::boards;
    use std::ops::Deref;

    let pool = pool.get().unwrap();
    let conn = pool.deref();

    for b in boards {
        let result: QueryResult<Board> = boards::dsl::boards.find(&b.short_name).first(conn);
        if result.is_err() {
            info!("creating board {:?}", b);
            diesel::insert(b)
                .into(boards::table)
                .execute(conn)
                .unwrap();
        }
    }
}

pub fn get_board(pool: Pool, board_name: &String) -> Option<Board> {
    use schema::boards;
    use std::ops::Deref;

    let pool = pool.get().unwrap();
    let conn = pool.deref();

    let result = boards::dsl::boards.find(board_name).first(conn);
    if let Ok(b) = result { Some(b) } else { None }
}

pub fn get_post_number(pool: Pool, board_name: &String) -> i64 {
    use schema::boards as board;
    use schema::boards::dsl::{boards, post_number};
    use std::ops::Deref;

    let pool = pool.get().unwrap();
    let conn = pool.deref();

    let b: Board = board::dsl::boards.find(board_name).first(conn).unwrap();
    let pnumber = b.post_number + 1;
    let _ = diesel::update(boards.find(board_name))
        .set(post_number.eq(pnumber))
        .get_result::<Board>(conn);

    pnumber
}

pub fn board_exists(pool: Pool, board_name: &String) -> bool {
    use schema::boards;
    use std::ops::Deref;

    let pool = pool.get().unwrap();
    let conn = pool.deref();

    let result: QueryResult<Board> = boards::dsl::boards.find(board_name).first(conn);
    result.is_ok()
}

pub fn create_thread(pool: Pool, thread: Post) {
    use schema::posts;
    use std::ops::Deref;

    let pool = pool.get().unwrap();
    let conn = pool.deref();

    diesel::insert(&thread)
        .into(posts::table)
        .execute(conn)
        .unwrap();
}
