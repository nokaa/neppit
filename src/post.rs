/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 *
 * Ἥφαιστος
 * */
use super::schema::posts;
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub post_number: i64,
    pub board: String,
    pub subject: Option<String>,
    pub name: String,
    pub email: String,
    pub content: String,
    pub thread: bool,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[table_name="posts"]
pub struct NewPost {
    pub board: String,
    pub subject: Option<String>,
    pub name: String,
    pub email: String,
    pub content: String,
    pub thread: bool,
}
