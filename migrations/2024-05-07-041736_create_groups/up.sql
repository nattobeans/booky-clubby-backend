CREATE TABLE groups (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    current_book_id integer NOT NULL REFERENCES books(id),
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
)