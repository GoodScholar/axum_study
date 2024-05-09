use std::env;

pub fn init_logger() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "tower_http=debug,middleware=debug");
    }
    tracing_subscriber::fmt::init();

    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env()
    //             .unwrap_or_else(|_| "example_jwt=debug".into()),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();
}
