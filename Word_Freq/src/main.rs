fn most_frequent_word(text: &str) -> (String, usize) {
    let mut word_count = std::collections::HashMap::new();

    for word in text.split_whitespace() {
        *word_count.entry(word.to_string()).or_insert(0) += 1;
    }

    let mut max_word = String::new();
    let mut max_count = 0;

    for (word, count) in word_count {
        if count > max_count {
            max_word = word;
            max_count = count;
        }
    }
    (max_word, max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}