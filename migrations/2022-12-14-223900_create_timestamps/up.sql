CREATE TABLE timestamps (
    -- https://www.postgresql.org/docs/current/datatype-character.html
    id VARCHAR(126) PRIMARY KEY,
    stamped_at TIMESTAMP NOT NULL
);
