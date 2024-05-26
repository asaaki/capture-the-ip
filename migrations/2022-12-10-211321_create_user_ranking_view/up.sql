-- refresh materialized view user_ranking;

CREATE MATERIALIZED VIEW user_ranking AS
WITH block_claims AS (
    SELECT
        nick,
        blck,
        COUNT(ip) AS claims,
        (rank() OVER (PARTITION BY blck ORDER BY COUNT(ip) DESC))::integer AS rank
    FROM captures
    GROUP BY nick, blck
),
max_rank AS (
    SELECT blck, max(rank)::integer AS max_rank
    FROM block_claims
	GROUP BY blck
),
ties AS (
    SELECT blck, rank, (CASE WHEN COUNT(nick) > 1 THEN true ELSE false END) AS is_tied
    FROM block_claims
    GROUP BY blck, rank
)
SELECT b.blck, b.rank, m.max_rank, t.is_tied, b.claims, b.nick
FROM block_claims b
JOIN max_rank m ON b.blck = m.blck
JOIN ties t ON b.blck = t.blck AND b.rank = t.rank
ORDER BY b.blck ASC, b.rank ASC;

ALTER MATERIALIZED VIEW user_ranking SET (autovacuum_vacuum_scale_factor = 0.0);
ALTER MATERIALIZED VIEW user_ranking SET (autovacuum_vacuum_threshold = 5000);
ALTER MATERIALIZED VIEW user_ranking SET (autovacuum_analyze_scale_factor = 0.0);
ALTER MATERIALIZED VIEW user_ranking SET (autovacuum_analyze_threshold = 5000);
