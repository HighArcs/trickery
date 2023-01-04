package Hangman;

import java.util.Scanner;

public class Hangman_runner {
    public static void main(String[] args) {
        final int max_guesses = 5;
        final String[] words = { "computer", "apple", "horse", "fallback", "closure", "option", "pointing" };
        final Scanner s = new Scanner(System.in);

        System.out.println(
                ansi_col(
                        "\nWelcome to Hangman! You have to guess the mystery word in " + max_guesses + " guesses or less",
                        "4"));

        final Hangman game = new Hangman(words[(int) (Math.random() * words.length)], max_guesses);

        while (game.can_guess() && !game.has_revealed_word()) {
            System.out.println("Here's what you have so far: " + game.revealed() + " (" + game.get_found_count() + "/"
                    + game.get_word().length() + " letters)");
            System.out.println("You have " + ansi_col("" + game.guesses_left(), "33") + " guesses left");
            System.out.println("Here are the letters you have guessed so far: " + game.guesses());
            System.out.print("What letter would you like to guess next? ");
            final String guess = s.next();

            System.out.print("\n");
            final boolean is_correct = game.guess_letter(guess);
            if (is_correct) {
                System.out.println(ansi_col("CORRECT!!!", "32"));
            } else {
                System.out.println(ansi_col("I'm sorry...", "31"));
            }
        }

        s.close();

        if (game.has_revealed_word()) {
            System.out.print(ansi_col("You win!", "33"));
        } else {
            System.out.print(ansi_col("You lost.", "33"));
        }

        System.out.println(" The word was: " + ansi_col(game.get_word(), "4"));
    }

    public static String ansi_col(String c, String code) {
        return "\u001b[" + code + "m" + c + "\u001b[0m";
    }
}
