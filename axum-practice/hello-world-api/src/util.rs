use tokio::net::TcpListener;

pub async fn bind_with_fallback(host: &str, port: u16) -> TcpListener {
    let mut port = port;
    let addr = format!("{host}:{port}");

    loop {
        match TcpListener::bind(&addr).await {
            Ok(listener) => {
                log::info!("SUCCESS: Binding To -> {}", addr);
                return listener;
            }
            Err(err) => {
                log::warn!(
                    "Failed to bind to {}\n {}\n Trying next port ...",
                    addr,
                    err
                );
                port = port
                    .checked_add(1)
                    .expect("Port Overflow while trying to find a free port.");
            }
        };
    }
}
