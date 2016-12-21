# Neppit [![Build Status](https://travis-ci.org/nokaa/neppit.svg?branch=master)](https://travis-ci.org/nokaa/neppit)

An anonymous Internet forum.

Neppit is also used as a test-bed for [hayaku](https://github.com/hayaku-rs/hayaku), and largely determines what features are developed.

### Install
Make sure that you have setup [Postgresql](https://wiki.archlinux.org/index.php/PostgreSQL).

```
$ createdb neppit
# replace `username` `password` and `localhost` as needed
$ echo "DATABASE_URL=postgres://username:password@localhost/neppit" > .env
$ cargo build --release
$ cargo run --release
```
