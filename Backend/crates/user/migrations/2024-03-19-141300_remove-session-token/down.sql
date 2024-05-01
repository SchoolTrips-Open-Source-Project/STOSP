-- This file should undo anything in `up.sql`
ALTER TABLE school_trips.users ADD COLUMN session_token Varchar;
ALTER TABLE school_trips.users 
    DROP COLUMN session_start,
    DROP COLUMN session_expiry;
