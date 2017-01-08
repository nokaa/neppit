use ::errors::*;
use ::board::Board;
use ::post::{Post, Thread};

use super::Pool;

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

pub fn get_catalog(pool: Pool, board: &Board) -> Result<Option<Vec<Thread>>> {
    let conn = pool.get().unwrap();

    let rows =
        conn.query("SELECT post_number, subject, name, content, pinned, last_modified FROM posts \
                    WHERE board = $1 AND active = $2",
                   &[&board.short_name, &true])?;
    if rows.is_empty() {
        Ok(None)
    } else {
        let mut threads = Vec::with_capacity(board.active_threads as usize);
        for row in rows.iter() {
            let post = Post {
                post_number: row.get(0),
                parent: row.get(0),
                board: board.short_name.clone(),
                subject: row.get(1),
                name: row.get(2),
                email: "".to_string(),
                content: row.get(3),
            };

            let thread = Thread {
                pinned: row.get(4),
                active: true,
                last_modified: row.get(5),
                post: post,
            };

            threads.push(thread);
        }
        Ok(Some(threads))
    }
}

// Retrieves all posts associated with a thread.
pub fn get_thread(pool: Pool, board_name: &str, thread_number: i64) -> Result<Vec<Post>> {
    let conn = pool.get().unwrap();
    // TODO(nokaa): Figure out why we need to do a wildcard select
    let rows =
        /*conn.query("SELECT (post_number, subject, name, email, content) FROM posts WHERE board = \
                    $1 AND parent = $2",
                   &[&board_name, &thread_number])?;*/
        conn.query("SELECT * FROM posts WHERE board = $1 AND parent = $2 AND active = true", &[&board_name, &thread_number])?;
    let mut thread = Vec::with_capacity(rows.len());
    for row in rows.iter() {
        let post = Post {
            post_number: row.get(0),
            parent: thread_number,
            board: board_name.to_string(),
            subject: row.get(3),
            name: row.get(4),
            email: row.get(5),
            content: row.get(6),
        };
        thread.push(post);
    }
    Ok(thread)
}
