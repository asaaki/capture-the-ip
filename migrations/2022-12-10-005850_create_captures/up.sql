CREATE TABLE captures (
    ip INTEGER PRIMARY KEY, -- 32 bit value
    blck SMALLINT NOT NULL, -- 16 bit value
    nick TEXT NOT NULL,
    claimed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX captures_by_nick ON captures (nick) INCLUDE (ip, blck, claimed_at);
CREATE INDEX captures_by_claimed_at ON captures (claimed_at DESC) INCLUDE (ip, nick);
CREATE INDEX captures_by_block ON captures (blck ASC) INCLUDE (ip, nick);

ALTER TABLE captures SET (autovacuum_vacuum_scale_factor = 0.0);
ALTER TABLE captures SET (autovacuum_vacuum_threshold = 5000);
ALTER TABLE captures SET (autovacuum_analyze_scale_factor = 0.0);
ALTER TABLE captures SET (autovacuum_analyze_threshold = 5000);
