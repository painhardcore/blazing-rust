#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub id: u64,
    pub ts: u64,
    pub kind: String,
}

/// Merges two event feeds into one timeline sorted by timestamp.
/// If an id appears more than once, the first occurrence (a before b) wins.
pub fn merge_events(a: &[Event], b: &[Event]) -> Vec<Event> {
    let mut out: Vec<Event> = Vec::new();
    for e in a.iter().chain(b.iter()) {
        if !out.iter().any(|o| o.id == e.id) {
            out.push(e.clone());
        }
    }

    let mut sorted = Vec::new();
    while !out.is_empty() {
        let mut min = 0;
        for i in 1..out.len() {
            if out[i].ts < out[min].ts {
                min = i;
            }
        }
        sorted.push(out.remove(min));
    }
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(id: u64, ts: u64, kind: &str) -> Event {
        Event { id, ts, kind: kind.to_string() }
    }

    #[test]
    fn merges_dedupes_and_sorts() {
        let a = vec![ev(1, 30, "a1"), ev(2, 10, "a2")];
        let b = vec![ev(2, 5, "b2"), ev(3, 20, "b3")];
        let got = merge_events(&a, &b);
        assert_eq!(got, vec![ev(2, 10, "a2"), ev(3, 20, "b3"), ev(1, 30, "a1")]);
    }
}
