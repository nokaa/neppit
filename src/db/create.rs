/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 * */
use errors::*;
use NewBoard;
use routes::types::*;
use super::Pool;

pub fn tables(pool: Pool) -> Result<()> {
    let conn = pool.get().unwrap();
    // Create the boards table
    conn.execute("CREATE TABLE IF NOT EXISTS boards (
                    short_name VARCHAR PRIMARY KEY,
                    long_name VARCHAR NOT NULL,
                    description TEXT NOT NULL,
                    post_number BIGINT NOT NULL
                  )",
                 &[])?;

    // Create the posts table
    conn.execute("CREATE TABLE IF NOT EXISTS posts (
                    post_number BIGINT NOT NULL,
                    parent BIGINT NOT NULL,
                    board VARCHAR NOT NULL,
                    subject VARCHAR,
                    content TEXT NOT NULL,
                    thread boolean NOT NULL,
                    last_modified TIMESTAMP,
                    PRIMARY KEY (board, post_number)
                  )",
                 &[])?;

    Ok(())
}

pub fn boards(pool: Pool, boards: &[NewBoard]) -> Result<()> {
    let conn = pool.get().unwrap();

    for b in boards {
        conn.execute("INSERT INTO boards VALUES ($1, $2, $3, $4) ON CONFLICT (short_name) DO \
                      NOTHING;",
                     &[&b.short_name, &b.long_name, &b.description, &0i64])?;
    }

    Ok(())
}

pub fn thread(pool: Pool, board: &str, thread: &NewThread) -> Result<u64> {
    let conn = pool.get().unwrap();

    let time = ::chrono::UTC::now().naive_utc();
    let rows =
        conn.query("with rows as (UPDATE boards SET post_number = post_number + 1 where \
                      short_name = $1 returning post_number) INSERT INTO posts VALUES ((SELECT \
                      post_number FROM rows), (SELECT post_number FROM rows), $1, $2, $3, $4, \
                      $5) RETURNING post_number;",
                     &[&board, &thread.subject, &thread.content, &true, &time])?;
    let post_number: i64 = rows.get(0).get(0);
    Ok(post_number as u64)
}

pub fn post(pool: Pool, board: &str, thread: i64, post: &NewPost) -> Result<u64> {
    let conn = pool.get().unwrap();

    let time = ::chrono::UTC::now().naive_utc();
    conn.execute("UPDATE posts SET last_modified = $1 where board = $2 and post_number = $3",
                 &[&time, &board, &thread])?;
    let rows =
        conn.query("with rows as (UPDATE boards SET post_number = post_number + 1 where \
                      short_name = $1 returning post_number) INSERT INTO posts VALUES ((SELECT \
                      post_number FROM rows), $2, $1, $3, $4, $5, \
                      $6) RETURNING post_number;",
                     &[&board, &thread, &None::<String>, &post.content, &false, &time])?;
    let post_number: i64 = rows.get(0).get(0);
    Ok(post_number as u64)
}
