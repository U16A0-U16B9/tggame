CREATE TABLE players (
   id UUID PRIMARY KEY,
   user_id VARCHAR UNIQUE NOT NULL,
   username VARCHAR NOT NULL
)