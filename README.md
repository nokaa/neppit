# Neppit [![Build Status](https://travis-ci.org/nokaa/neppit.svg?branch=master)](https://travis-ci.org/nokaa/neppit)

An anonymous Internet forum.

Neppit is also used as a test-bed for [hayaku](https://github.com/hayaku-rs/hayaku),
and largely determines what features are developed.

### Install
Make sure that you have setup [Postgresql](https://wiki.archlinux.org/index.php/PostgreSQL).

[Sass](https://github.com/sass/sassc) is used for our stylesheets, make sure
that you have `sassc` installed if you wish to build the css.

We use [just](https://github.com/casey/just) for convenience, you can install
just with `cargo install just`. If you do not wish to install just, just run
the commands in `Justfile`. Just syntax is very similar to make.

```
$ createdb neppit
# replace `username` `password` and `localhost` as needed
$ echo "DATABASE_URL=postgres://username:password@localhost/neppit" > .env
$ just
$ cargo build --release
$ cargo run --release
```
