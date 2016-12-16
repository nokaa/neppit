# senatus

An anonymous Internet forum.

Senatus is also used as a test-bed for [hayaku](https://github.com/nokaa/hayaku), and largely determines what features are developed.

### Install
Make sure that you have setup [Postgresql](https://wiki.archlinux.org/index.php/PostgreSQL).

```
# Install diesel-cli
$ cargo install diesel_cli
$ createdb senatus
# replace `username` `password` and `localhost` as needed
$ echo "DATABASE_URL=postgres://username:password@localhost/senatus" > .env
$ diesel migration run
$ cargo build --release
$ cargo run --release
```
