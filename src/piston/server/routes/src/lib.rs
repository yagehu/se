mod namespace;

use axum::Router;

pub fn router<S: Clone + Send + Sync + 'static>(state: S) -> Router<S> {
    Router::new().nest("/namespace", namespace::router(state))
}
