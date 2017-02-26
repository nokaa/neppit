# Neppit [![Build Status](https://travis-ci.org/nokaa/neppit.svg?branch=master)](https://travis-ci.org/nokaa/neppit)

An anonymous Internet forum. You can see it in action at
[https://forum.nokaa.moe](https://forum.nokaa.moe).

Neppit is intended as a forum for software projects; an alternative to other options such as _discourse_.

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
  - Images: Image boards tend to attract unsavory content, not hosting images makes admin work much easier

### Wish List
These are features that would be nice to have, but that I might not work on.
If you would like to work on one of the features, just open an issue about
which feature you would like to work on.

  - Better SQL
  - Basic Moderation: Login for post deletion, bans, etc.
  - Board and post links
  - Link parsing
  - Improved styling
  - API: Probably a JSON API, maybe look to 4chan for an example
  - Minor bug fixes
    - Post date display should only show e.g. 2017-01-01 00:00:01
    - You can attempt to load a post that doesn't exist, e.g. go to [https://forum.nokaa.moe/b/news/1000000](https://forum.nokaa.moe/b/news/1000000)
    - Entering the url to a reply (`/reply`) should load `/thread#reply`, currently loads the thread data at `/reply` which affects new post creation.
