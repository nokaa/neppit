/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 *
 * Ἥφαιστος
 * */
use super::schema::boards;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    pub short_name: String,
    pub long_name: String,
    pub description: String,
    pub post_number: i64,
    // The currently active threads for this board
    pub active_threads: Vec<i64>,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[table_name="boards"]
pub struct NewBoard {
    pub short_name: String,
    long_name: String,
    description: String,
    active_threads: Vec<i64>,
}

impl Board {
    pub fn thread_is_active(&self, thread: &String) -> bool {
        if !self.active_threads.is_empty() {
            let thread = i64::from_str_radix(thread, 10).unwrap();
            for &t in &self.active_threads {
                if t == thread {
                    return true;
                }
            }
        }
        false
    }
}
