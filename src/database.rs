use r2d2;
use r2d2_postgres::PostgresConnectionManager;

use super::errors::*;
use super::board::{Board, NewBoard};
use super::moderator::Mod;
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

    // Create the admins table
    conn.execute("CREATE TABLE IF NOT EXISTS admins (
                    username VARCHAR PRIMARY KEY,
                    password VARCHAR NOT NULL,
                    boards VARCHAR[] NOT NULL,
                    admin boolean NOT NULL
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

pub fn create_admin(pool: Pool, admin: Mod) -> Result<()> {
    let conn = pool.get().unwrap();

    conn.execute("INSERT INTO admins (username, password, boards, admin) VALUES ($1, $2, $3, $4)",
                 &[&admin.username, &admin.password, &admin.boards, &admin.admin])?;
    Ok(())
}

pub fn create_boards(pool: Pool, boards: &[NewBoard]) -> Result<()> {
    let conn = pool.get().unwrap();

    for b in boards {
        conn.execute("INSERT INTO boards (short_name, long_name, description, post_number, \
                      active_threads) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (short_name) DO \
                      NOTHING",
                     &[&b.short_name, &b.long_name, &b.description, &0i64, &Vec::<i64>::new()])?;
    }

    Ok(())
}

pub fn get_board(pool: Pool, board_name: &str) -> Result<Option<Board>> {
    let conn = pool.get().unwrap();

    let rows =
        conn.query("SELECT short_name, long_name, description, post_number, active_threads \
                    FROM boards WHERE short_name = $1",
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

    let rows =
        conn.query("UPDATE boards SET post_number = post_number + 1 WHERE short_name = $1 \
                    RETURNING post_number",
                   &[&board_name])?;
    Ok(rows.get(0).get(0))
}

pub fn board_exists(pool: Pool, board_name: &str) -> Result<bool> {
    let conn = pool.get().unwrap();

    // TODO(nokaa): Look at the EXISTS keyword for this check:
    // https://stackoverflow.com/questions/7471625/fastest-check-if-row-exists-in-postgresql
    let rows = conn.query("SELECT short_name FROM boards WHERE short_name = $1",
               &[&board_name])?;
    Ok(!rows.is_empty())
}

pub fn thread_exists(pool: Pool, board_name: &str, thread_number: i64) -> Result<bool> {
    let conn = pool.get().unwrap();

    // TODO(nokaa): Look at the EXISTS keyword for this check:
    // https://stackoverflow.com/questions/7471625/fastest-check-if-row-exists-in-postgresql
    let rows =
        conn.query("SELECT post_number, board, thread FROM posts WHERE board = $1 AND \
                    post_number = $2 AND thread = $3",
                   &[&board_name, &thread_number, &true])?;
    Ok(!rows.is_empty())
}

pub fn create_thread(pool: Pool, thread: Post) -> Result<()> {
    let conn = pool.get().unwrap();
    let trans = conn.transaction()?;

    trans.execute("INSERT INTO posts (post_number, board, subject, name, email, content, thread, \
                  parent) VALUES ($1, $2, $3, $4, $5, $6, $7, $8);",
                 &[&thread.post_number,
                   &thread.board,
                   &thread.subject,
                   &thread.name,
                   &thread.email,
                   &thread.content,
                   &thread.thread,
                   &thread.parent])?;

    trans.execute("UPDATE boards SET active_threads = $1::BIGINT || active_threads WHERE \
                  short_name = $2",
                 &[&thread.post_number, &thread.board])?;

    trans.commit()?;
    Ok(())
}

pub fn get_thread(pool: Pool, board_name: &str, thread_number: i64) -> Result<Option<Post>> {
    let conn = pool.get().unwrap();

    let rows =
        conn.query("SELECT post_number, board, subject, name, email, content, thread, parent \
                    FROM posts WHERE board = $1 AND post_number = $2 AND thread = $3",
                   &[&board_name, &thread_number, &true])?;

    if rows.is_empty() {
        Ok(None)
    } else {
        let row = rows.get(0);
        let thread = Post {
            post_number: row.get(0),
            board: row.get(1),
            subject: row.get(2),
            name: row.get(3),
            email: row.get(4),
            content: row.get(5),
            thread: row.get(6),
            parent: row.get(7),
        };
        Ok(Some(thread))
    }
}

pub fn create_post(pool: Pool, post: Post) -> Result<()> {
    let conn = pool.get().unwrap();

    conn.execute("INSERT INTO posts (post_number, board, subject, name, email, content, thread, \
                   parent) VALUES ($1, $2, $3, $4, $5, $6, $7, $8);",
                 &[&post.post_number,
                   &post.board,
                   &post.subject,
                   &post.name,
                   &post.email,
                   &post.content,
                   &post.thread,
                   &post.parent])?;
    Ok(())
}

pub fn get_posts(pool: Pool, board_name: &str, thread_number: i64) -> Result<Option<Vec<Post>>> {
    let conn = pool.get().unwrap();
    let rows =
        conn.query("SELECT post_number, board, subject, name, email, content, thread, parent \
                    FROM posts WHERE board = $1 AND parent = $2 AND thread = $3",
                   &[&board_name, &thread_number, &false])?;
    if rows.is_empty() {
        Ok(None)
    } else {
        let mut posts = Vec::with_capacity(rows.len());
        for row in rows.iter() {
            let post = Post {
                post_number: row.get(0),
                board: row.get(1),
                subject: row.get(2),
                name: row.get(3),
                email: row.get(4),
                content: row.get(5),
                thread: row.get(6),
                parent: row.get(7),
            };
            posts.push(post);
        }
        Ok(Some(posts))
    }
}
