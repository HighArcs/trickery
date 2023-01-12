pub fn activity_one<const T: usize>(
    mut words: [&'static str; T],
    new_word: &'static str,
    place: usize,
) -> String {
    if place >= words.len() {
        return "you need a valid number".to_string();
    }

    let mut prev = new_word;

    for i in place..words.len() {
        let current = prev;
        prev = words[i];
        words[i] = current;
    }

    let mut output = "".to_owned();

    for i in 0..words.len() {
        output += words[i];
    }

    output
}

pub fn activity_two_swap<const T: usize>(mut vec: [i32; T], i: usize, j: usize) {
    if !(i > vec.len() || j > vec.len()) {
        let t = vec[i];
        vec[i] = vec[j];
        vec[j] = t;
    }
}

pub fn activity_two_reverse_swap<const T: usize>(mut a: [i32; T]) {
    for i in 0..a.len() / 2 {
        let t = a[i];
        a[i] = a[a.len() - i - 1];
        a[a.len() - i - 1] = t;
    }
}
