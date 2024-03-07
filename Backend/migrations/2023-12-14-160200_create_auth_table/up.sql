-- Your SQL goes here
CREATE TABLE school_trips.auth
    (
    id VARCHAR(255) PRIMARY KEY,
    mobile_number VARCHAR(20) NOT NULL UNIQUE,
    country_code VARCHAR(10) NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    otp VARCHAR(10) NOT NULL,
    updated_at  TIMESTAMP NOT NULL
)