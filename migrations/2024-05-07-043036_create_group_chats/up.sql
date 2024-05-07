CREATE TABLE group_chats (
    id SERIAL PRIMARY KEY,
    message  text NOT NULL,
    group_id integer NOT NULL REFERENCES groups(id),
    to_member_id integer NOT NULL REFERENCES members(id),
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
)