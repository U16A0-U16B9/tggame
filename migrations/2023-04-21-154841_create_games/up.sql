CREATE TYPE status AS ENUM ('looking_for_group', 'in_progress', 'completed');
CREATE TYPE time_of_day AS ENUM ('dawn', 'day', 'dusk', 'night');
CREATE TABLE games (
    id UUID PRIMARY KEY,
    chat_id VARCHAR NOT NULL,
    status status NOT NULL,
    time_of_day time_of_day DEFAULT 'dusk' NOT NULL
)