#[axum::debug_handler]
pub async fn sentry_error() {
    panic!("Sentry backend error test")
}
