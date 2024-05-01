-- This file should undo anything in `up.sql`
create schema if not exists school_trips
alter table school_trips_user.user rename school_trips.user
alter table school_trips_user.auth rename school_trips.auth
drop schema if exists school_trips_user