use std::collections::HashMap;

/// Counts requests per "METHOD STATUS" from access log lines like:
/// `2026-11-01T12:00:00Z GET /api/users 200 12ms`
/// Lines with fewer than five fields are skipped.
pub fn status_counts(input: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    for line in lines {
        let fields: Vec<String> = line.split_whitespace().map(|f| f.to_string()).collect();
        if fields.len() < 5 {
            continue;
        }
        let key = format!("{} {}", fields[1].clone(), fields[3].clone());
        let current = counts.get(&key).cloned().unwrap_or(0);
        counts.insert(key.clone(), current + 1);
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_method_and_status() {
        let input = "t GET /a 200 1ms\nt POST /b 500 2ms\nt GET /c 200 3ms\nbroken line\n";
        let counts = status_counts(input);
        assert_eq!(counts.len(), 2);
        assert_eq!(counts["GET 200"], 2);
        assert_eq!(counts["POST 500"], 1);
    }

    #[test]
    fn empty_input() {
        assert!(status_counts("").is_empty());
    }
}
