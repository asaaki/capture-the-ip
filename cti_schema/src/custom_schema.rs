// block holders view

// select blck, nick, claims, latest_claim

diesel::table! {
    use diesel::sql_types::*;

    block_holders (blck) {
        blck -> Int2,
        is_tied -> Bool,
        nick -> Text,
        claims -> Int8,
    }
}

// global rankings views

// select nick, array_agg(blck order by blck asc) as blocks, sum(claims)::bigint as total_claims

diesel::table! {
    use diesel::sql_types::*;

    ranking_all_time (nick) {
        nick -> Text,
        blocks -> Array<Int2>,
        total_claims -> Int8,
    }
}
diesel::table! {
    use diesel::sql_types::*;

    ranking_year (nick) {
        nick -> Text,
        blocks -> Array<Int2>,
        total_claims -> Int8,
    }
}
diesel::table! {
    use diesel::sql_types::*;

    ranking_month (nick) {
        nick -> Text,
        blocks -> Array<Int2>,
        total_claims -> Int8,
    }
}
diesel::table! {
    use diesel::sql_types::*;

    ranking_week (nick) {
        nick -> Text,
        blocks -> Array<Int2>,
        total_claims -> Int8,
    }
}
diesel::table! {
    use diesel::sql_types::*;

    ranking_day (nick) {
        nick -> Text,
        blocks -> Array<Int2>,
        total_claims -> Int8,
    }
}
diesel::table! {
    use diesel::sql_types::*;

    ranking_hour (nick) {
        nick -> Text,
        blocks -> Array<Int2>,
        total_claims -> Int8,
    }
}

// user ranking view

// select b.blck, b.rank, m.max_rank, t.is_tied, b.claims, b.nick
diesel::table! {
    use diesel::sql_types::*;

    user_ranking (blck) {
        blck -> Int2,
        rank -> Int4,
        max_rank -> Int4,
        is_tied -> Bool,
        claims -> Int8,
        nick -> Text,
    }
}

// for "show max_connections", but SELECTable

diesel::table! {
    use diesel::sql_types::*;

    // table has more columns, but we only need the name and setting
    pg_settings (name) {
        name -> Text,
        setting -> Text,
    }
}

// for getting current connection count

diesel::table! {
    use diesel::sql_types::*;

    // table has more columns, but we only need the few here
    pg_stat_activity (pid) {
        pid -> Integer,
        #[sql_name = "datname"]
        database -> Text,
        #[sql_name = "usename"]
        username -> Text,
    }
}
