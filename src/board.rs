/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 *
 * Ἥφαιστος
 * */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    pub short_name: String,
    pub long_name: String,
    pub description: String,
    pub active_threads: i32,
    pub post_number: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewBoard {
    pub short_name: String,
    pub long_name: String,
    pub description: String,
    pub active_threads: i32,
}
