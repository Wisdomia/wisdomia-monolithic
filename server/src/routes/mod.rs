mod dbg;
mod wisdoms;

use axum::Router;

pub fn routes() -> Router {
    Router::new().merge(_routes())
}

fn _routes() -> Router {
    let mut router = Router::new();
    router = router.merge(dbg::routes()).merge(wisdoms::routes());
    router
}
