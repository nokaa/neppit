use postgres;

error_chain!{
    foreign_links {
        PostGres(postgres::error::Error);
    }
}
