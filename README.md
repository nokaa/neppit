# Neppit [![Build Status](https://travis-ci.org/nokaa/neppit.svg?branch=master)](https://travis-ci.org/nokaa/neppit)

An anonymous Internet forum.

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
$ just run-release
```

### Non-Goals
  - User Logins: Neppit is anonymous
  - Shadow banning: It's a disgusting practice

### Wish List
These are features that would be nice to have, but that I might not work on.
If you would like to work on one of the features, just open an issue about
which feature you would like to work on.

  - Better SQL
  - Basic Moderation: Login for post deletion, possibly IP ban
  - Greentext
  - Minor bug fixes
