/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 *
 * Ἥφαιστος
 * */
use post::Post;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thread {
    pub thread_number: usize,
    subject: String,
    name: String,
    email: String,
    content: String,
    children: Vec<Post>,
}

pub struct NewThread {
    pub subject: String,
    pub name: String,
    pub email: String,
    pub content: String,
    pub children: Vec<Post>,
}
