/*

NOTE: The following are materialized views which need to get manually updated!

refresh materialized view ranking_all_time;
refresh materialized view ranking_year;
refresh materialized view ranking_month;
refresh materialized view ranking_week;
refresh materialized view ranking_day;
refresh materialized view ranking_hour;

*/

-- TOP ALL TIME

CREATE MATERIALIZED VIEW ranking_all_time AS
WITH block_claims AS (
    SELECT nick, blck, claims
    FROM (
        SELECT
            nick,
            blck,
            count(ip) AS claims,
            rank() OVER (PARTITION BY blck ORDER BY count(ip) DESC) AS rank
        FROM captures
        GROUP BY nick, blck
    ) block_claims
    WHERE rank = 1
    ORDER BY blck ASC
),
ties AS (
    SELECT blck
    FROM block_claims
    GROUP BY blck
    HAVING COUNT(*) > 1
),
filtered AS (
    SELECT *
    FROM block_claims
    WHERE blck NOT IN (SELECT * FROM ties)
),
results AS (
    SELECT nick, array_agg(blck ORDER BY blck ASC) AS blocks, sum(claims)::bigint AS total_claims
    FROM filtered
    GROUP BY nick
)
SELECT * FROM results
ORDER BY array_length(blocks, 1) DESC, total_claims DESC;

-- TOP YEAR

CREATE MATERIALIZED VIEW ranking_year AS
WITH block_claims AS (
    SELECT nick, blck, claims
    FROM (
        SELECT
            nick,
            blck,
            count(ip) AS claims,
            rank() OVER (PARTITION BY blck ORDER BY count(ip) DESC) AS rank
        FROM captures
        WHERE claimed_at >= now() - interval '1 year'
        GROUP BY nick, blck
    ) block_claims
    WHERE rank = 1
    ORDER BY blck ASC
),
ties AS (
    SELECT blck
    FROM block_claims
    GROUP BY blck
    HAVING COUNT(*) > 1
),
filtered AS (
    SELECT *
    FROM block_claims
    WHERE blck NOT IN (SELECT * FROM ties)
),
results AS (
    SELECT nick, array_agg(blck ORDER BY blck ASC) AS blocks, sum(claims)::bigint AS total_claims
    FROM filtered
    GROUP BY nick
)
SELECT * FROM results
ORDER BY array_length(blocks, 1) DESC, total_claims DESC;

-- TOP MONTH

CREATE MATERIALIZED VIEW ranking_month AS
WITH block_claims AS (
    SELECT nick, blck, claims
    FROM (
        SELECT
            nick,
            blck,
            count(ip) AS claims,
            rank() OVER (PARTITION BY blck ORDER BY count(ip) DESC) AS rank
        FROM captures
        WHERE claimed_at >= now() - interval '1 month'
        GROUP BY nick, blck
    ) block_claims
    WHERE rank = 1
    ORDER BY blck ASC
),
ties AS (
    SELECT blck
    FROM block_claims
    GROUP BY blck
    HAVING COUNT(*) > 1
),
filtered AS (
    SELECT *
    FROM block_claims
    WHERE blck NOT IN (SELECT * FROM ties)
),
results AS (
    SELECT nick, array_agg(blck ORDER BY blck ASC) AS blocks, sum(claims)::bigint AS total_claims
    FROM filtered
    GROUP BY nick
)
SELECT * FROM results
ORDER BY array_length(blocks, 1) DESC, total_claims DESC;

-- TOP WEEK

CREATE MATERIALIZED VIEW ranking_week AS
WITH block_claims AS (
    SELECT nick, blck, claims
    FROM (
        SELECT
            nick,
            blck,
            count(ip) AS claims,
            rank() OVER (PARTITION BY blck ORDER BY count(ip) DESC) AS rank
        FROM captures
        WHERE claimed_at >= now() - interval '1 week'
        GROUP BY nick, blck
    ) block_claims
    WHERE rank = 1
    ORDER BY blck ASC
),
ties AS (
    SELECT blck
    FROM block_claims
    GROUP BY blck
    HAVING COUNT(*) > 1
),
filtered AS (
    SELECT *
    FROM block_claims
    WHERE blck NOT IN (SELECT * FROM ties)
),
results AS (
    SELECT nick, array_agg(blck ORDER BY blck ASC) AS blocks, sum(claims)::bigint AS total_claims
    FROM filtered
    GROUP BY nick
)
SELECT * FROM results
ORDER BY array_length(blocks, 1) DESC, total_claims DESC;

-- TOP DAY

CREATE MATERIALIZED VIEW ranking_day AS
WITH block_claims AS (
    SELECT nick, blck, claims
    FROM (
        SELECT
            nick,
            blck,
            count(ip) AS claims,
            rank() OVER (PARTITION BY blck ORDER BY count(ip) DESC) AS rank
        FROM captures
        WHERE claimed_at >= now() - interval '1 day'
        GROUP BY nick, blck
    ) block_claims
    WHERE rank = 1
    ORDER BY blck ASC
),
ties AS (
    SELECT blck
    FROM block_claims
    GROUP BY blck
    HAVING COUNT(*) > 1
),
filtered AS (
    SELECT *
    FROM block_claims
    WHERE blck NOT IN (SELECT * FROM ties)
),
results AS (
    SELECT nick, array_agg(blck ORDER BY blck ASC) AS blocks, sum(claims)::bigint AS total_claims
    FROM filtered
    GROUP BY nick
)
SELECT * FROM results
ORDER BY array_length(blocks, 1) DESC, total_claims DESC;

-- TOP HOUR

CREATE MATERIALIZED VIEW ranking_hour AS
WITH block_claims AS (
    SELECT nick, blck, claims
    FROM (
        SELECT
            nick,
            blck,
            count(ip) AS claims,
            rank() OVER (PARTITION BY blck ORDER BY count(ip) DESC) AS rank
        FROM captures
        WHERE claimed_at >= now() - interval '1 hour'
        GROUP BY nick, blck
    ) block_claims
    WHERE rank = 1
    ORDER BY blck ASC
),
ties AS (
    SELECT blck
    FROM block_claims
    GROUP BY blck
    HAVING COUNT(*) > 1
),
filtered AS (
    SELECT *
    FROM block_claims
    WHERE blck NOT IN (SELECT * FROM ties)
),
results AS (
    SELECT nick, array_agg(blck ORDER BY blck ASC) AS blocks, sum(claims)::bigint AS total_claims
    FROM filtered
    GROUP BY nick
)
SELECT * FROM results
ORDER BY array_length(blocks, 1) DESC, total_claims DESC;

-- vacuum

ALTER MATERIALIZED VIEW ranking_all_time SET (autovacuum_vacuum_scale_factor = 0.0);
ALTER MATERIALIZED VIEW ranking_all_time SET (autovacuum_vacuum_threshold = 5000);
ALTER MATERIALIZED VIEW ranking_all_time SET (autovacuum_analyze_scale_factor = 0.0);
ALTER MATERIALIZED VIEW ranking_all_time SET (autovacuum_analyze_threshold = 5000);

ALTER MATERIALIZED VIEW ranking_day SET (autovacuum_vacuum_scale_factor = 0.0);
ALTER MATERIALIZED VIEW ranking_day SET (autovacuum_vacuum_threshold = 5000);
ALTER MATERIALIZED VIEW ranking_day SET (autovacuum_analyze_scale_factor = 0.0);
ALTER MATERIALIZED VIEW ranking_day SET (autovacuum_analyze_threshold = 5000);

ALTER MATERIALIZED VIEW ranking_hour SET (autovacuum_vacuum_scale_factor = 0.0);
ALTER MATERIALIZED VIEW ranking_hour SET (autovacuum_vacuum_threshold = 5000);
ALTER MATERIALIZED VIEW ranking_hour SET (autovacuum_analyze_scale_factor = 0.0);
ALTER MATERIALIZED VIEW ranking_hour SET (autovacuum_analyze_threshold = 5000);

ALTER MATERIALIZED VIEW ranking_month SET (autovacuum_vacuum_scale_factor = 0.0);
ALTER MATERIALIZED VIEW ranking_month SET (autovacuum_vacuum_threshold = 5000);
ALTER MATERIALIZED VIEW ranking_month SET (autovacuum_analyze_scale_factor = 0.0);
ALTER MATERIALIZED VIEW ranking_month SET (autovacuum_analyze_threshold = 5000);

ALTER MATERIALIZED VIEW ranking_week SET (autovacuum_vacuum_scale_factor = 0.0);
ALTER MATERIALIZED VIEW ranking_week SET (autovacuum_vacuum_threshold = 5000);
ALTER MATERIALIZED VIEW ranking_week SET (autovacuum_analyze_scale_factor = 0.0);
ALTER MATERIALIZED VIEW ranking_week SET (autovacuum_analyze_threshold = 5000);

ALTER MATERIALIZED VIEW ranking_year SET (autovacuum_vacuum_scale_factor = 0.0);
ALTER MATERIALIZED VIEW ranking_year SET (autovacuum_vacuum_threshold = 5000);
ALTER MATERIALIZED VIEW ranking_year SET (autovacuum_analyze_scale_factor = 0.0);
ALTER MATERIALIZED VIEW ranking_year SET (autovacuum_analyze_threshold = 5000);
