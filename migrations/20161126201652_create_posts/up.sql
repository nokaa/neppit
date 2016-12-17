CREATE TABLE posts (
       post_number BIGINT NOT NULL,
       board VARCHAR NOT NULL,
       subject VARCHAR,
       name VARCHAR NOT NULL,
       email VARCHAR NOT NULL,
       content TEXT NOT NULL,
       thread BOOLEAN NOT NULL,
       parent BIGINT,
       PRIMARY KEY (board, post_number)
)
