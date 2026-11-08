use crate::clock::Clock;

#[tokio::test]
async fn clock_defaults_cover_now() {
    let _ = Clock::new().now();
    let _ = Clock::default().uuid();
}
