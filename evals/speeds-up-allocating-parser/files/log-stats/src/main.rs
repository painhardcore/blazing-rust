use std::{env, fs};

fn main() {
    let path = env::args().nth(1).expect("usage: log-stats <access.log>");
    let input = fs::read_to_string(&path).expect("read log");
    let mut counts: Vec<_> = log_stats::status_counts(&input).into_iter().collect();
    counts.sort();
    for (key, n) in counts {
        println!("{key}\t{n}");
    }
}
