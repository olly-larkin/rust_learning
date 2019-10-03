use std::collections::HashMap;

#[derive(Default, Debug, Clone, Copy)]
struct Stats {
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}

pub fn tally(match_results: &str) -> String {
    let mut result = String::from("Team                           | MP |  W |  D |  L |  P");

    let match_results: Vec<Vec<&str>> = match_results   // collect input in manageable form
        .split(|c| c == '\n')
        .map(|string| 
            string.split(|c| c == ';')
                .collect::<Vec<&str>>()
        )
        .collect();

    let mut hash: HashMap<&str, Stats> = HashMap::new();

    for m in &match_results {
        hash.entry(m[0]).or_default().mp += 1;
        hash.entry(m[1]).or_default().mp += 1;
        match m[2] {
            "win" => {
                hash.entry(m[0]).or_default().w += 1;
                hash.entry(m[0]).or_default().p += 3;
                hash.entry(m[1]).or_default().l += 1;
            },
            "loss" => {
                hash.entry(m[1]).or_default().w += 1;
                hash.entry(m[1]).or_default().p += 3;
                hash.entry(m[0]).or_default().l += 1;
            },
            "draw" => {
                hash.entry(m[0]).or_default().d += 1;
                hash.entry(m[0]).or_default().p += 1;
                hash.entry(m[1]).or_default().d += 1;
                hash.entry(m[1]).or_default().p += 1;
            },
            _ => unreachable!(),
        }
    }

    let mut sorted_hash = hash.iter().collect::<Vec<_>>();
    sorted_hash.sort_by(|a, b| if a.1.p == b.1.p {
        a.0.cmp(b.0)            // if score is the same, sort by name
    } else {
        b.1.p.cmp(&a.1.p)       // sort by score
    });

    for (&name, &s) in sorted_hash.iter() {
        result.push_str(format!("\n{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", name, s.mp, s.w, s.d, s.l, s.p).as_str());
    }

    result
}
