/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 * */
use r2d2;
use r2d2_postgres::PostgresConnectionManager;

pub mod create;
pub mod read;

pub type Pool = r2d2::Pool<PostgresConnectionManager>;

#[derive(Serialize, Deserialize)]
pub struct CatalogItem {
    pub post_number: i64,
    pub board: String,
    pub subject: String,
    pub content: String,
    pub last_modified: ::chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub subject: Option<String>,
    pub content: String,
    pub post_number: i64,
    pub last_modified: ::chrono::NaiveDateTime,
}
