-- Your SQL goes here
CREATE TABLE school_trips.user
    (
    id VARCHAR(255) PRIMARY KEY,
    user_name VARCHAR(30) UNIQUE,
    mobile_number VARCHAR(20) UNIQUE,
    country_code VARCHAR(10),
    created_at  TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL,
    user_pass VARCHAR(20) CHECK(length(user_pass) >= 8)
)