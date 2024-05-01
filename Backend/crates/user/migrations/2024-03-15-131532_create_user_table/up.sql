-- Your SQL goes here
CREATE TYPE role_type AS ENUM ('PARENT', 'DRIVER');

CREATE TABLE school_trips.users 
    ( name  Varchar(255), 
    created_at  TIMESTAMPTZ NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL,
    session_token  Varchar NOT NULL,
    mobile_number  Varchar NOT NULL,
    role role_type NOT NULL,
    id Varchar PRIMARY KEY 
    );