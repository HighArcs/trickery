package Hangman;

public class Hangman {
    private String word;
    private int max_guesses;
    // takes less space than a list of `char`s
    private String correct_guesses = "";
    private String wrong_guesses = "";

    // create a new Hangman instance
    public Hangman(String word, int max_guesses) {
        this.word = word;
        this.max_guesses = max_guesses;
    }

    // converts the string to lowercase, lets users not care about capitalization
    public static Hangman with_ignore_case(String word, int max_guesses) {
        return new Hangman(word.toLowerCase(), max_guesses);
    }

    // guesses a letter, returning false if it is not in the word
    public boolean guess_letter(String letter) {
        // validate
        if (letter.length() != 1 || this.wrong_guesses.length() >= this.max_guesses) {
            return false;
        }

        // if we've already guessed that it is in the word, then we don't need to care
        if (this.correct_guesses.contains(letter)) {
            return true;
        }

        if (this.wrong_guesses.contains(letter)) {
            return false; // it'll show that it is wrong, but we have already guessed this
        }

        final boolean contains = this.word.contains(letter);

        if (contains) {
            correct_guesses += letter;
        } else {
            wrong_guesses += letter;
        }

        return contains;
    }

    // guess a letter without caring about casing. this will only work if we use Hangman.with_ignore_case
    public boolean guess_letter_ignore_case(String letter) {
        return this.guess_letter(letter.toLowerCase());
    }

    // get the currently revealed string, replacing unknown chars with a red '*'
    public String revealed() {
        String out = "";

        // walk tge strubg
        for (int i = 0; i < this.word.length(); i++) {
            // get current character
            final String c = this.word.substring(i, i + 1);
            // if we have it, make it a green version of the letter, terminal magic
            if (correct_guesses.contains(c)) {
                out += "\u001b[32m" + c + "\u001b[0m";
            } else {
                out += "\u001b[31m*\u001b[0m";
            }
        }

        return out;
    }

    // list all of our guesses
    public String guesses() {
        String out = "";

        // list of correct letters, in green
        for (int i = 0; i < this.correct_guesses.length(); i++) {
            final String c = this.correct_guesses.substring(i, i + 1);
            out += "\u001b[32m" + c + "\u001b[0m ";
        }

        // wrong letters, in red
        for (int i = 0; i < this.wrong_guesses.length(); i++) {
            final String c = this.wrong_guesses.substring(i, i + 1);
            out += "\u001b[31m" + c + "\u001b[0m ";
        }

        // trim extra spaces at the end, since we always add ' ' at the end of each letter
        while (out.endsWith(" ")) {
            out = out.substring(0, out.length() - 1);
        }

        return out;
    }

    public boolean can_guess() {
        // only count wrong_guesses, correct guesses do not count toward the guess count
        return this.wrong_guesses.length() < this.max_guesses;
    }

    public int guesses_left() {
        return this.max_guesses - this.wrong_guesses.length();
    }

    // checking if the player has won
    public boolean has_revealed_word() {
        for (int i = 0; i < this.word.length(); i++) {
            final String c = this.word.substring(i, i + 1);
            if (!correct_guesses.contains(c)) {
                return false;
            }

            // no else here, wait until the entire string is done to ensure that we have it all
        }

        return true;
    }

    public String get_word() {
        return this.word;
    }

    // not the same as correct_guesses.length(), this counts duplicate letters ('cc' would be counted as 2, not 1)
    public int get_found_count() {
        int count = 0;
        for (int i = 0; i < this.word.length(); i++) {
            final String c = this.word.substring(i, i + 1);
            if (correct_guesses.contains(c)) {
                count++;
            }
        }

        return count;
    }

}
