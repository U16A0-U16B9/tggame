CREATE TABLE roles (
    id UUID PRIMARY KEY,
    name VARCHAR UNIQUE NOT NULL,
    description VARCHAR NOT NULL,
    win_condition VARCHAR NOT NULL
)