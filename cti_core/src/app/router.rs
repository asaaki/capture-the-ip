use super::{api::*, helpers::*, r#static::*};
use crate::prelude::*;
use axum::{
    routing::{any, get},
    Router,
};

#[cfg(debug_assertions)]
use super::debug::*;

pub(crate) fn router(pool: DbPool) -> Router {
    let state: AppState = pool;

    let users_routes = Router::new().route("/", get(user_ranking));
    let ranks_routes = Router::new()
        .route("/all", get(get_ranking_all_time))
        .route("/year", get(get_ranking_year))
        .route("/month", get(get_ranking_month))
        .route("/week", get(get_ranking_week))
        .route("/day", get(get_ranking_day))
        .route("/hour", get(get_ranking_hour))
        // default root if no period is selected
        .route("/", get(get_ranking_week))
        .with_state(state.clone());

    let api_routes = Router::new()
        .nest("/users", users_routes)
        .nest("/ranks", ranks_routes)
        .route("/blocks", get(get_block_holders))
        .route("/recent", get(get_recent_claims))
        .route("/ip", get(get_ip))
        .fallback(handler_404)
        .with_state(state.clone());

    #[allow(unused_mut)]
    let mut app_router = Router::new()
        .route("/claim", any(claim_ip))
        .nest("/api", api_routes);

    #[cfg(debug_assertions)]
    {
        let debug_routes = Router::new()
            .route("/info", get(request_info))
            .route("/seed", get(seed_handler))
            .with_state(state.clone());

        app_router = app_router.nest("/debug", debug_routes);
    };

    app_router.fallback(static_handler).with_state(state)
}
