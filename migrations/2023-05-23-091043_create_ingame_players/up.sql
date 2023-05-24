CREATE TABLE ingame_players (
   id UUID PRIMARY KEY,
   game_id UUID NOT NULL REFERENCES games(id),
   player_id UUID NOT NULL REFERENCES players(id),
   role_id UUID REFERENCES roles(id),
   is_alive BOOLEAN NOT NULL DEFAULT TRUE
)-- Your SQL goes here