CREATE TABLE boards (
       short_name VARCHAR PRIMARY KEY,
       long_name VARCHAR NOT NULL,
       description TEXT NOT NULL,
       post_number SERIAL NOT NULL,
       active_threads INTEGER[] NOT NULL
)
