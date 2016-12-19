#!/bin/bash

psql -d neppit -c "DROP TABLE boards"
psql -d neppit -c "DROP TABLE posts"
