//! Writes N synthetic access log lines to stdout: `cargo run --release --example gen_log -- 2000000 > access.log`
use std::io::{self, BufWriter, Write};

fn main() {
    let n: u64 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(1_000_000);
    let methods = ["GET", "GET", "GET", "POST", "PUT", "DELETE"];
    let statuses = ["200", "200", "200", "201", "304", "404", "500"];
    let paths = ["/api/users", "/api/orders/42", "/static/app.js", "/health", "/api/search?q=rust"];
    let mut out = BufWriter::new(io::stdout().lock());
    let mut x: u64 = 0x9E3779B97F4A7C15;
    for i in 0..n {
        x ^= x << 13; x ^= x >> 7; x ^= x << 17;
        let m = methods[(x % methods.len() as u64) as usize];
        let s = statuses[((x >> 8) % statuses.len() as u64) as usize];
        let p = paths[((x >> 16) % paths.len() as u64) as usize];
        writeln!(out, "2026-11-01T12:{:02}:{:02}Z {m} {p} {s} {}ms", (i / 60) % 60, i % 60, x % 900).unwrap();
    }
}
