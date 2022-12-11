create table timestamps (
    -- https://www.postgresql.org/docs/current/datatype-character.html
    id varchar(126) primary key,
    stamped_at timestamp not null
);
