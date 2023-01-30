pub fn activity_one(list: Vec<String>, letter: char) -> u32 {
    let mut count = 0;
    for word in list {
        if word[1].eq_ignore_ascii_case(letter) {
            count += 1;
        }
    }

    count
}

pub fn activity_two(vec: Vec<String>, item: String) -> isize {
    for (i, word) in vec.iter().enumerate() {
        if word == item {
            for (j, other) in vec.iter().skip(i).enumerate() {
                if other == item {
                    return j as isize;
                }
            }
        }
    }

    return -1;
}