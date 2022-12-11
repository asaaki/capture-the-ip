create table captures (
    ip integer primary key, -- 32 bit value
    blck smallint not null, -- 16 bit value
    nick text not null,
    claimed_at timestamp not null default CURRENT_TIMESTAMP
);
create index captures_by_nick on captures (nick) include (ip, blck, claimed_at);
create index captures_by_claimed_at on captures (claimed_at DESC) include (ip, nick);
create index captures_by_block on captures (blck ASC) include (ip, nick);
