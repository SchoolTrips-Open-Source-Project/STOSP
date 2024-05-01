-- Your SQL goes here

ALTER TABLE school_trips.auth ADD COLUMN token_expiry TIMESTAMP NOT NULL DEFAULT now()