CREATE TABLE group_members (
    id SERIAL PRIMARY KEY,
    user_id  integer NOT NULL REFERENCES members(id),
    group_id integer NOT NULL REFERENCES groups(id),
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
)