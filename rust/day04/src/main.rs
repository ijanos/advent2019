fn no_large_group(pw: &[u8]) -> bool {
    let mut last = pw[0];
    let mut last_count = 1;
    for &p in &pw[1..] {
        if p == last {
            last_count += 1;
        } else {
            if last_count == 2 {
                return true;
            }
            last = p;
            last_count = 1;
        }
    }
    last_count == 2
}

fn main() {
    let two_same = |pw: &[u8]| pw.windows(2).any(|w| w[0] == w[1]);
    let never_dec = |pw: &[u8]| pw.windows(2).all(|w| w[0] <= w[1]);

    let part1 = (138241..=674034).map(|n| n.to_string()).filter(|pw| two_same(pw.as_bytes()) && never_dec(pw.as_bytes())).count();
    let part2 = (138241..=674034).map(|n| n.to_string()).filter(|pw| two_same(pw.as_bytes()) && never_dec(pw.as_bytes()) && no_large_group(pw.as_bytes())).count();

    println!("Part 2: {}", part1);
    println!("Part 2: {}", part2);
}
