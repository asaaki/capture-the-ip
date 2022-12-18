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

create materialized view ranking_all_time as
with block_claims as (
    select nick, blck, claims
    from (
        select
            nick,
            blck,
            count(ip) as claims,
            rank() over (partition by blck order by count(ip) desc) as rank
        from captures
        group by nick, blck
    ) block_claims
    where rank = 1
    order by blck asc
),
ties as (
    select blck
    from block_claims
    group by blck
    having count(*) > 1
),
filtered as (
    select *
    from block_claims
    where blck not in (select * from ties)
),
results as (
    select nick, array_agg(blck order by blck asc) as blocks, sum(claims)::bigint as total_claims
    from filtered
    group by nick
)
select * from results
order by array_length(blocks, 1) desc, total_claims desc;

-- TOP YEAR

create materialized view ranking_year as
with block_claims as (
    select nick, blck, claims
    from (
        select
            nick,
            blck,
            count(ip) as claims,
            rank() over (partition by blck order by count(ip) desc) as rank
        from captures
        where claimed_at >= now() - interval '1 year'
        group by nick, blck
    ) block_claims
    where rank = 1
    order by blck asc
),
ties as (
    select blck
    from block_claims
    group by blck
    having count(*) > 1
),
filtered as (
    select *
    from block_claims
    where blck not in (select * from ties)
),
results as (
    select nick, array_agg(blck order by blck asc) as blocks, sum(claims)::bigint as total_claims
    from filtered
    group by nick
)
select * from results
order by array_length(blocks, 1) desc, total_claims desc;

-- TOP MONTH

create materialized view ranking_month as
with block_claims as (
    select nick, blck, claims
    from (
        select
            nick,
            blck,
            count(ip) as claims,
            rank() over (partition by blck order by count(ip) desc) as rank
        from captures
        where claimed_at >= now() - interval '1 month'
        group by nick, blck
    ) block_claims
    where rank = 1
    order by blck asc
),
ties as (
    select blck
    from block_claims
    group by blck
    having count(*) > 1
),
filtered as (
    select *
    from block_claims
    where blck not in (select * from ties)
),
results as (
    select nick, array_agg(blck order by blck asc) as blocks, sum(claims)::bigint as total_claims
    from filtered
    group by nick
)
select * from results
order by array_length(blocks, 1) desc, total_claims desc;

-- TOP WEEK

create materialized view ranking_week as
with block_claims as (
    select nick, blck, claims
    from (
        select
            nick,
            blck,
            count(ip) as claims,
            rank() over (partition by blck order by count(ip) desc) as rank
        from captures
        where claimed_at >= now() - interval '1 week'
        group by nick, blck
    ) block_claims
    where rank = 1
    order by blck asc
),
ties as (
    select blck
    from block_claims
    group by blck
    having count(*) > 1
),
filtered as (
    select *
    from block_claims
    where blck not in (select * from ties)
),
results as (
    select nick, array_agg(blck order by blck asc) as blocks, sum(claims)::bigint as total_claims
    from filtered
    group by nick
)
select * from results
order by array_length(blocks, 1) desc, total_claims desc;

-- TOP DAY

create materialized view ranking_day as
with block_claims as (
    select nick, blck, claims
    from (
        select
            nick,
            blck,
            count(ip) as claims,
            rank() over (partition by blck order by count(ip) desc) as rank
        from captures
        where claimed_at >= now() - interval '1 day'
        group by nick, blck
    ) block_claims
    where rank = 1
    order by blck asc
),
ties as (
    select blck
    from block_claims
    group by blck
    having count(*) > 1
),
filtered as (
    select *
    from block_claims
    where blck not in (select * from ties)
),
results as (
    select nick, array_agg(blck order by blck asc) as blocks, sum(claims)::bigint as total_claims
    from filtered
    group by nick
)
select * from results
order by array_length(blocks, 1) desc, total_claims desc;

-- TOP HOUR

create materialized view ranking_hour as
with block_claims as (
    select nick, blck, claims
    from (
        select
            nick,
            blck,
            count(ip) as claims,
            rank() over (partition by blck order by count(ip) desc) as rank
        from captures
        where claimed_at >= now() - interval '1 hour'
        group by nick, blck
    ) block_claims
    where rank = 1
    order by blck asc
),
ties as (
    select blck
    from block_claims
    group by blck
    having count(*) > 1
),
filtered as (
    select *
    from block_claims
    where blck not in (select * from ties)
),
results as (
    select nick, array_agg(blck order by blck asc) as blocks, sum(claims)::bigint as total_claims
    from filtered
    group by nick
)
select * from results
order by array_length(blocks, 1) desc, total_claims desc;
