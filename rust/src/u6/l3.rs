pub fn activity_one<const T: usize>(words: [&str; T]) -> Option<&str> {
    let mut output: Option<&str> = None;

    for i in 0..words.len() {
        if output.is_none() || words[i].len() < output.unwrap().len() {
            output = Some(words[i]);
        }
    }

    return output;
}

pub fn activity_two<const T: usize>(words: [&str; T]) {
    for i in 0..words.len() {
        for j in 0..words[i].len() {
            let slice = &words[i][j..j + 1];

            if !(slice == "a" || slice == "e" || slice == "i" || slice == "o" || slice == "u") {
                print!("{slice}");
            }
        }

        print!("\n");
    }
}

pub fn activity_three<const T: usize>(vec: [&str; T]) {
    for i in 0..vec.len() {
        if vec[i].len() >= 2 && &vec[i][0..2] == "un" {
            println!("{}", vec[i])
        }
    }
}
