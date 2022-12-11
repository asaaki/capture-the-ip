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
