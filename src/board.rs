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
    short_name: String,
    long_name: String,
    description: String,
    post_number: i32,
    // The currently active threads for this board
    active_threads: Vec<i32>,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[table_name="boards"]
pub struct NewBoard {
    pub short_name: String,
    long_name: String,
    description: String,
    active_threads: Vec<i32>,
}

impl Board {
    pub fn thread_is_active(&self, thread: &String) -> bool {
        if !self.active_threads.is_empty() {
            let thread = i32::from_str_radix(thread, 10).unwrap();
            for &t in &self.active_threads {
                if t == thread {
                    return true;
                }
            }
        }
        false
    }
}
