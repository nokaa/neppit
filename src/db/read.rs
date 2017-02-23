/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 * */
use super::{CatalogItem, Pool, Post};
use errors::*;

pub fn catalog(pool: Pool, board: &str) -> Result<Vec<CatalogItem>> {
    let conn = pool.get().unwrap();

    let rows = conn.query("SELECT * FROM posts WHERE board = $1 AND thread = $2 ORDER \
                          BY last_modified",
               &[&board, &true])?;

    if rows.is_empty() {
        Ok(Vec::new())
    } else {
        let mut catalog = Vec::with_capacity(rows.len());
        for row in rows.iter() {
            let subject: Option<String> = row.get(3);
            let item = CatalogItem {
                post_number: row.get(0),
                board: row.get(2),
                subject: subject.unwrap(),
                content: row.get(4),
                last_modified: row.get(6),
            };
            catalog.push(item);
        }
        Ok(catalog)
    }
}

pub fn thread(pool: Pool, board: &str, thread_num: i64) -> Result<Vec<Post>> {
    let conn = pool.get().unwrap();

    let rows = conn.query("SELECT * FROM posts WHERE board = $1 AND parent = $2",
                          &[&board, &thread_num])?;

    if rows.is_empty() {
        let rows = conn.query("SELECT * FROM posts WHERE board = $1 AND post_number = $2",
                              &[&board, &thread_num])?;
        let parent = rows.get(0).get(1);
        thread(pool, board, parent)
    } else {
        let mut thread = Vec::with_capacity(rows.len());
        for row in rows.iter() {
            let post = Post {
                subject: row.get(3),
                content: row.get(4),
                post_number: row.get(0),
                last_modified: row.get(6),
            };
            thread.push(post);
        }
        Ok(thread)
    }
}
