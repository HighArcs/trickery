pub fn activity_one(mut f: impl std::io::Write, word: String) -> String {
    if word.len() <= 1 {
        return word;
    }

    return activity_one(word[1..].to_owned()) + &word[0..1]
}