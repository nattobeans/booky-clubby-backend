CREATE TABLE reviews (
    id SERIAL PRIMARY KEY,
    review text NOT NULL,
    member_id integer NOT NULL REFERENCES members(id),
    book_id integer NOT NULL REFERENCES books(id),
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
)