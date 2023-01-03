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

    public static Hangman with_ignore_case(String word, int max_guesses) {
        return new Hangman(word.toLowerCase(), max_guesses);
    }

    // guesses a letter, returning false if it is not in the word
    public boolean guess_letter(String letter) {
        // validate
        if (letter.length() != 1 || this.wrong_guesses.length() >= this.max_guesses) {
            return false;
        }

        if (this.correct_guesses.contains(letter)) {
            return true;
        }

        final boolean contains = this.word.contains(letter);

        if (contains) {
            correct_guesses += letter;
        } else {
            wrong_guesses += letter;
        }

        return contains;
    }

    public boolean guess_letter_ignore_case(String letter) {
        return this.guess_letter(letter.toLowerCase());
    }

    public String revealed() {
        String out = "";

        for (int i = 0; i < this.word.length(); i++) {
            final String c = this.word.substring(i, i + 1);
            if (correct_guesses.contains(c)) {
                out += c;
            } else {
                out += "*";
            }
        }

        return out;
    }

    public String guesses() {
        final String[] total_guesses = (this.correct_guesses + this.wrong_guesses).split(".");

        // sort alphabetically
        java.util.Arrays.sort(total_guesses);

        return String.join(" ", total_guesses);
    }
}
