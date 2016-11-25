use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use super::schema;
use super::board::{Board, NewBoard};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error conntecting to {}", database_url))
}

pub fn create_boards(conn: &PgConnection, boards: &[NewBoard]) {
    use schema::boards;

    for b in boards {
        let result: QueryResult<Board> = boards::dsl::boards.find(&b.short_name).first(conn);
        if result.is_err() {
            info!("creating board {:?}", b);
            diesel::insert(b)
                .into(boards::table)
                .execute(conn)
                .unwrap();
        }
    }
    // get_result(conn)
    // expect("Error saving new board")
}
