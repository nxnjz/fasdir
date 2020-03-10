//todo check that it works as expected
pub fn split_evenly<T: std::clone::Clone>(items: Vec<T>, into: usize) -> Vec<Vec<T>> {
    let mut counts = Vec::new();
    for _ in 0..(items.len() % into) {
        counts.push(items.len() / into + 1);
    }
    for _ in 0..(into - (items.len() % into)) {
        counts.push(items.len() / into);
    }

    let mut split = Vec::new();
    let (mut s, mut e) = (0, 0);
    for i in 0..into {
        let count = counts[i];
        e = e + count;
        split.push(items[s..e].to_vec());
        s = e;
    }
    return split;
}
