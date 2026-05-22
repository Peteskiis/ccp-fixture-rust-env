//! E2E fixture for `ccp compute deploy` env-precedence tests in
//! Peteskiis/cluster-infra (#165).
//!
//! Exposes two routes:
//!   GET /env/<KEY>  -> plain-text value of std::env::var(KEY), empty if unset
//!   GET /health     -> 200 OK
//!
//! Listens on $PORT (defaults to 3000). Stdlib-only to keep cold builds in
//! the ~30s range — adding tokio/axum would push the e2e tier past its
//! ~10-12 min budget. The fixture is intentionally minimal HTTP — just
//! enough to read a path and write a body. No keep-alive, no streaming.

use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).unwrap_or_else(|e| panic!("bind {addr}: {e}"));
    eprintln!("listening on {addr}");
    for stream in listener.incoming().flatten() {
        thread::spawn(move || handle(stream));
    }
}

fn handle(mut stream: TcpStream) {
    let mut buf = [0u8; 4096];
    let n = match stream.read(&mut buf) {
        Ok(n) if n > 0 => n,
        _ => return,
    };
    // Parse just the request-line: `GET /path HTTP/1.1`. We don't care about
    // headers or body for these test routes.
    let request = String::from_utf8_lossy(&buf[..n]);
    let path = request.split_whitespace().nth(1).unwrap_or("/");
    let (status, body) = if path == "/health" {
        ("200 OK", String::new())
    } else if let Some(key) = path.strip_prefix("/env/") {
        // Strip any query string just in case — keep this minimal but robust.
        let key = key.split('?').next().unwrap_or("");
        ("200 OK", env::var(key).unwrap_or_default())
    } else {
        ("404 Not Found", String::new())
    };
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(response.as_bytes());
}
