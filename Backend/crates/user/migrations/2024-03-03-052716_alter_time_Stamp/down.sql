-- This file should undo anything in `up.sql`
ALTER TABLE school_trips.auth ALTER COLUMN created_at SET DATA TYPE TIMESTAMP;
ALTER TABLE school_trips.auth ALTER COLUMN updated_at SET DATA TYPE TIMESTAMP;
ALTER TABLE school_trips.auth ALTER COLUMN token_expiry SET DATA TYPE TIMESTAMP;