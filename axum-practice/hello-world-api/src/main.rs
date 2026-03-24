mod app;
mod util;

use app::create_app;
use axum::serve;

use crate::util::bind_with_fallback;

#[tokio::main]
async fn main() {
    serve(bind_with_fallback("0.0.0.0", 3000).await, create_app())
        .await
        .expect("Failed to start axum server")
}
