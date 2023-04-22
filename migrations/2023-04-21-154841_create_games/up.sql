CREATE TYPE status AS ENUM ('looking_for_group', 'in_progress', 'completed');
CREATE TABLE games (
    id UUID PRIMARY KEY,
    chat_id VARCHAR NOT NULL,
    status status NOT NULL
)