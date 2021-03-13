use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

pub fn  ssl_config() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(
            "privkey.pem",
            SslFiletype::PEM,
        )
        .expect("ho345fd");
    builder
        .set_certificate_chain_file("fullchain.pem")
        .expect("hi53gs");
    builder
}

pub fn rate_limit(
    max_request: usize,
    time_limit: u64,
    store: MemoryStore,
) -> RateLimiter<MemoryStoreActor> {
    RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
        .with_interval(std::time::Duration::from_secs(time_limit))
        .with_max_requests(max_request)
}

// pub fn  ssl_config() -> SslAcceptorBuilder {
//     let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
//     builder
//         .set_private_key_file(
//             "/etc/letsencrypt/live/adbuyengine.com/privkey.pem",
//             SslFiletype::PEM,
//         )
//         .expect("ho345fd");
//     builder
//         .set_certificate_chain_file("/etc/letsencrypt/live/adbuyengine.com/fullchain.pem")
//         .expect("hi53gs");
//     builder
// }