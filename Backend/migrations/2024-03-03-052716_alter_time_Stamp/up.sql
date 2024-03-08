-- Your SQL goes here
ALTER TABLE school_trips.auth ALTER COLUMN created_at SET DATA TYPE TIMESTAMPTZ;
ALTER TABLE school_trips.auth ALTER COLUMN updated_at SET DATA TYPE TIMESTAMPTZ;
ALTER TABLE school_trips.auth ALTER COLUMN token_expiry SET DATA TYPE TIMESTAMPTZ;