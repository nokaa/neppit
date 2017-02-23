/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 * */
#[derive(FromForm)]
pub struct NewThread {
    pub subject: String,
    pub content: String,
}

#[derive(FromForm)]
pub struct NewPost {
    pub content: String,
}
