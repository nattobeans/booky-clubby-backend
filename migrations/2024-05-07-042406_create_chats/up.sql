CREATE TABLE chats (
    id SERIAL PRIMARY KEY,
    message  text NOT NULL,
    member_id integer NOT NULL REFERENCES members(id),
    to_member_id integer NOT NULL REFERENCES members(id),
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
)