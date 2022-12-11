-- refresh materialized view user_ranking;

create materialized view user_ranking as
with block_claims as (
    select
        nick,
        blck,
        count(ip) as claims,
        (rank() over (partition by blck order by count(ip) desc))::integer as rank
    from captures
    group by nick, blck
),
max_rank as (
    select blck, max(rank)::integer as max_rank
    from block_claims
	group by blck
),
ties as (
    select blck, rank, (case when count(nick) > 1 then true else false end) as is_tied
    from block_claims
    group by blck, rank
)
select b.blck, b.rank, m.max_rank, t.is_tied, b.claims, b.nick
from block_claims b
join max_rank m on b.blck = m.blck
join ties t on b.blck = t.blck and b.rank = t.rank
order by b.blck asc, b.rank asc
