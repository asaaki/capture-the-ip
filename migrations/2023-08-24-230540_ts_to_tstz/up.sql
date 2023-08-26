SET timezone = 'UTC';

alter table timestamps
alter stamped_at type timestamptz;

alter table captures
alter claimed_at type timestamptz;
