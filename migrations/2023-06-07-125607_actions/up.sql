
CREATE TYPE action AS ENUM ('kill');
CREATE TABLE actions (
   id UUID PRIMARY KEY,
   action action NOT NULL,
   ingame_player_id UUID NOT NULL REFERENCES ingame_players(id),
   time_of_day time_of_day NOT NULL ,
   completed BOOLEAN DEFAULT false NOT NULL
)