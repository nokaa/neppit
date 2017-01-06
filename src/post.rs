/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 *
 * Ἥφαιστος
 * */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub post_number: i64,
    pub parent: i64,
    pub board: String,
    // Only applicable to thread creation, replies to a thread do not have
    // a subject.
    pub subject: Option<String>,
    pub name: String,
    pub email: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thread {
    /// Marks if this thread is pinned or not.
    pub pinned: bool,
    /// Marks if this post is active. Only applies if post is a thread.
    pub active: bool,
    pub last_modified: ::chrono::NaiveDateTime,
    pub post: Post,
}
