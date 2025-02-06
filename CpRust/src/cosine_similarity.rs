use std::collections::BTreeMap;

fn cosine_similarity(d1: &str, d2: &str) -> f64 {
    let mut md1: BTreeMap<String, i32> = BTreeMap::new();
    let mut md2: BTreeMap<String, i32> = BTreeMap::new();

    let vd1: Vec<String> = d1.split_whitespace().map(|i| i.to_string()).collect();
    let vd2: Vec<String> = d2.split_whitespace().map(|i| i.to_string()).collect();

    for word in vd1 {
        *md1.entry(word).or_insert(0) += 1;
    }

    for word in vd2 {
        *md2.entry(word).or_insert(0) += 1;
    }

    let dot_product: i32 = md1
        .iter()
        .filter_map(|(word, &count1)| md2.get(word).map(|&count2| count1.min(count2)))
        .sum();

    let mag1: f64 = md1
        .values()
        .map(|&count| (count * count) as f64)
        .sum::<f64>()
        .sqrt();

    let mag2: f64 = md2
        .values()
        .map(|&count| (count * count) as f64)
        .sum::<f64>()
        .sqrt();

    if mag1 == 0.0 || mag2 == 0.0 {
        return 0.0;
    }

    dot_product as f64 / (mag1 * mag2)
}

fn main() {
    let d1 = "the best data science course";
    let d2 = "data science is popular";
    println!("{}", cosine_similarity(d1, d2));
}
