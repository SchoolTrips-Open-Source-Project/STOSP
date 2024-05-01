-- Your SQL goes here
CREATE SCHEMA IF NOT EXISTS school_trips_driver;
CREATE TABLE school_trips_driver.auth
    (
    id VARCHAR(255) PRIMARY KEY,
    mobile_number VARCHAR NOT NULL,
    country_code VARCHAR(10) NOT NULL,
    token VARCHAR NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    otp VARCHAR(10) NOT NULL,
    updated_at  TIMESTAMP NOT NULL
);

ALTER TABLE school_trips_driver.auth ADD COLUMN token_expiry TIMESTAMP NOT NULL DEFAULT now();
ALTER TABLE school_trips_driver.auth ALTER COLUMN created_at SET DATA TYPE TIMESTAMPTZ;
ALTER TABLE school_trips_driver.auth ALTER COLUMN updated_at SET DATA TYPE TIMESTAMPTZ;
ALTER TABLE school_trips_driver.auth ALTER COLUMN token_expiry SET DATA TYPE TIMESTAMPTZ;

CREATE TYPE school_trips_driver.role_type AS ENUM ('PARENT', 'DRIVER');

CREATE TABLE school_trips_driver.users 
    ( name  Varchar(255), 
    created_at  TIMESTAMPTZ NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL,
    session_token  Varchar NOT NULL,
    mobile_number  Varchar NOT NULL,
    role role_type NOT NULL,
    id Varchar PRIMARY KEY 
    );

ALTER TABLE school_trips_driver.auth ADD COLUMN role role_type NOT NULL;

ALTER TABLE school_trips_driver.users DROP COLUMN session_token;