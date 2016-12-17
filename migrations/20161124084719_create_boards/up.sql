CREATE TABLE boards (
       short_name VARCHAR PRIMARY KEY,
       long_name VARCHAR NOT NULL,
       description TEXT NOT NULL,
       post_number BIGINT NOT NULL,
       active_threads BIGINT[] NOT NULL
)
