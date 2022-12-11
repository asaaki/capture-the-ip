#[global_allocator]
static ALLOC: rpmalloc::RpMalloc = rpmalloc::RpMalloc;

use cti_core::app_prelude::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> AppResult {
    run_app(RunMode::Dual).await
}
