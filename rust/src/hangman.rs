pub struct Hangman {
    word: String,
    max_guesses: u32,
    correct_guesses: String,
    wrong_guesses: String,
}

impl Hangman {
    pub fn new(word: String, max_guesses: u32) -> Self {
        Self {
            correct_guesses: String::new(),
            max_guesses,
            word,
            wrong_guesses: String::new(),
        }
    }

    pub fn with_ignore(word: String, max_guesses: u32) -> Self {
        Self::new(word.to_lowercase(), max_guesses)
    }

    pub fn guess_letter(&mut self, letter: String) -> bool {
        if letter.len() != 1 || self.wrong_guesses.len() >= (self.max_guesses as usize) {
            return false;
        }

        if self.correct_guesses.contains(&letter) {
            return true;
        }

        if self.wrong_guesses.contains(&letter) {
            return false;
        }

        let contains = self.word.contains(&letter);

        if contains {
            self.correct_guesses += &letter;
        } else {
            self.wrong_guesses += &letter;
        }

        contains
    }

    pub fn guess_letter_ignore_case(&mut self, letter: String) -> bool {
        self.guess_letter(letter.to_lowercase())
    }

    pub fn revealed(&self) -> String {
        let mut out = String::new();

        for i in 0..self.word.len() {
            let c = &self.word[i..i + 1];

            if self.correct_guesses.contains(c) {
                out += "\u{001b}[32m";
                out += c;
                out += "\u{001b}[0m";
            } else {
                out += "\u{001b}[31m*\u{001b}[0m";
            }
        }

        out
    }

    pub fn guesses(&self) -> String {
        let mut out = String::new();

        for i in 0..self.correct_guesses.len() {
            let c = &self.correct_guesses[i..i + 1];
            out += "\u{001b}[32m";
            out += c;
            out += "\u{001b}[0m ";
        }

        for i in 0..self.wrong_guesses.len() {
            let c = &self.wrong_guesses[i..i + 1];
            out += "\u{001b}[31m";
            out += c;
            out += "\u{001b}[0m ";
        }

        out.trim_end().to_owned()
    }

    pub fn can_guess(&self) -> bool {
        (self.wrong_guesses.len() as u32) < self.max_guesses
    }

    pub fn guesses_left(&self) -> u32 {
        self.max_guesses - (self.wrong_guesses.len() as u32)
    }

    pub fn has_revealed_word(&self) -> bool {
        for i in 0..self.word.len() {
            let c = &self.word[i..i + 1];
            if !self.correct_guesses.contains(c) {
                return false;
            }
        }

        return false;
    }

    pub fn get_word(&self) -> &String {
        &self.word
    }

    pub fn get_found_count(&self) -> u32 {
        let mut count = 0;
        for i in 0..self.word.len() {
            let c = &self.word[i..i + 1];
            if self.correct_guesses.contains(c) {
                count += 1;
            }
        }

        count
    }


}
