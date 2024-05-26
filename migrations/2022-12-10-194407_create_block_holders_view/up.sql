-- refresh materialized view block_holders;

CREATE materialized view block_holders AS
WITH block_claims AS (
    SELECT
        nick,
        blck,
        COUNT(ip) AS claims,
        (rank() OVER (PARTITION BY blck ORDER BY count(ip) DESC))::integer AS rank
    FROM captures
    GROUP BY nick, blck
),
ties AS (
    SELECT blck, rank, (CASE WHEN count(nick) > 1 THEN true ELSE false END) AS is_tied
    FROM block_claims
    GROUP BY blck, rank
)
SELECT DISTINCT b.blck, t.is_tied, (CASE WHEN t.is_tied THEN 'tied' ELSE b.nick END) AS nick, b.claims
FROM block_claims b
JOIN ties t ON b.blck = t.blck AND b.rank = t.rank
WHERE b.rank = 1
ORDER BY b.blck ASC;

ALTER MATERIALIZED VIEW block_holders SET (autovacuum_vacuum_scale_factor = 0.0);
ALTER MATERIALIZED VIEW block_holders SET (autovacuum_vacuum_threshold = 5000);
ALTER MATERIALIZED VIEW block_holders SET (autovacuum_analyze_scale_factor = 0.0);
ALTER MATERIALIZED VIEW block_holders SET (autovacuum_analyze_threshold = 5000);
