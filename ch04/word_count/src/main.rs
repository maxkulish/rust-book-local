use std::collections::HashMap;

fn main() {
    let text = "hello world hello axs Nestor nestor";

    let mut freqs = HashMap::new();

    for word in text.split_whitespace() {
        *freqs.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    println!("Word frequencies: {:#?}", freqs);
}
