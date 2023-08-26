SET timezone = 'UTC';

alter table timestamps
alter stamped_at type timestamp;

alter table captures
alter claimed_at type timestamp;
