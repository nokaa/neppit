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
    pub post_number: usize,
    pub content: String,
    pub name: String,
    pub email: String,
}
