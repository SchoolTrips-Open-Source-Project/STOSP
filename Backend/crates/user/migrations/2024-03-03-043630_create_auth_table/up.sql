-- Your SQL goes here
CREATE SCHEMA IF NOT EXISTS school_trips;
CREATE TABLE school_trips.auth
    (
    id VARCHAR(255) PRIMARY KEY,
    mobile_number VARCHAR NOT NULL,
    country_code VARCHAR(10) NOT NULL,
    token VARCHAR NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    otp VARCHAR(10) NOT NULL,
    updated_at  TIMESTAMP NOT NULL
)