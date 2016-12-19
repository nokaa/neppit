use r2d2;
use r2d2_postgres::PostgresConnectionManager;

use super::errors::*;
use super::board::{Board, NewBoard};
use super::post::Post;

pub type Pool = r2d2::Pool<PostgresConnectionManager>;

pub fn create_tables(pool: Pool) -> Result<()> {
    let conn = pool.get().unwrap();
    // Create the boards table
    conn.execute("CREATE TABLE IF NOT EXISTS boards (
                    short_name VARCHAR PRIMARY KEY,
                    long_name VARCHAR NOT NULL,
                    description TEXT NOT NULL,
                    post_number BIGINT NOT NULL,
                    active_threads BIGINT[] NOT NULL
                  )",
                 &[])?;

    // Create the posts table
    conn.execute("CREATE TABLE IF NOT EXISTS posts (
                    post_number BIGINT NOT NULL,
                    board VARCHAR NOT NULL,
                    subject VARCHAR,
                    name VARCHAR NOT NULL,
                    email VARCHAR NOT NULL,
                    content TEXT NOT NULL,
                    thread boolean NOT NULL,
                    parent BIGINT,
                    PRIMARY KEY (board, post_number)
                  )",
                 &[])?;

    Ok(())
}

pub fn create_boards(pool: Pool, boards: &[NewBoard]) -> Result<()> {
    let conn = pool.get().unwrap();

    for b in boards {
        let rows = conn.query("SELECT short_name FROM boards WHERE short_name = $1",
                   &[&b.short_name])?;
        if rows.is_empty() {
            info!("creating board {:?}", b);
            conn.execute("INSERT INTO boards (short_name, long_name, description, post_number, \
                          active_threads) VALUES ($1, $2, $3, $4, $5)",
                         &[&b.short_name,
                           &b.long_name,
                           &b.description,
                           &0i64,
                           &Vec::<i64>::new()])?;
        }
    }

    Ok(())
}

pub fn get_board(pool: Pool, board_name: &str) -> Result<Option<Board>> {
    let conn = pool.get().unwrap();

    let rows = conn.query("SELECT short_name FROM boards WHERE short_name = $1",
               &[&board_name])?;
    if rows.is_empty() {
        Ok(None)
    } else {
        let row = rows.get(0);
        let board = Board {
            short_name: row.get(0),
            long_name: row.get(1),
            description: row.get(2),
            post_number: row.get(3),
            active_threads: row.get(4),
        };
        Ok(Some(board))
    }
}

pub fn get_post_number(pool: Pool, board_name: &str) -> Result<i64> {
    let conn = pool.get().unwrap();

    let rows = conn.query("UPDATE boards SET post_number = post_number + 1 WHERE board_name = $1",
               &[&board_name])?;
    Ok(rows.get(0).get(3))
}

pub fn board_exists(pool: Pool, board_name: &str) -> Result<bool> {
    let conn = pool.get().unwrap();

    let rows = conn.query("SELECT short_name FROM boards WHERE short_name = $1",
               &[&board_name])?;
    Ok(rows.is_empty())
}

pub fn create_thread(pool: Pool, thread: Post) -> Result<()> {
    let conn = pool.get().unwrap();

    conn.execute("INSERT INTO posts (post_number, board, subject, name, email, content, thread, \
                  parent) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                 &[&thread.post_number,
                   &thread.board,
                   &thread.subject,
                   &thread.name,
                   &thread.email,
                   &thread.content,
                   &thread.thread,
                   &thread.parent])?;
    Ok(())
}
