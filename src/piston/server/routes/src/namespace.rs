use axum::{routing::post, Router};

pub fn router<S: Clone + Send + Sync + 'static>(state: S) -> Router<S> {
    Router::new().route("/", post(create)).with_state(state)
}

async fn create() {
}
