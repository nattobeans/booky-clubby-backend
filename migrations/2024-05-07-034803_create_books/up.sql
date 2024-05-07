CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description text NOT NULL,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
)