-- refresh materialized view block_holders;

create materialized view block_holders as
with block_claims as (
    select
        nick,
        blck,
        count(ip) as claims,
        (rank() over (partition by blck order by count(ip) desc))::integer as rank
    from captures
    group by nick, blck
),
ties as (
    select blck, rank, (case when count(nick) > 1 then true else false end) as is_tied
    from block_claims
    group by blck, rank
)
select distinct b.blck, t.is_tied, (case when t.is_tied then 'tied' else b.nick end) as nick, b.claims
from block_claims b
join ties t on b.blck = t.blck and b.rank = t.rank
where b.rank = 1
order by b.blck asc;
