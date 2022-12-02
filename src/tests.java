import java.util.Scanner;

import shapes.Circle;
import shapes.Rectangle;

public class tests {
    public static void main(String[] args) {
        Book b1 = new Book("George Orwell", "Nineteen Eighty-Four", 1949);
        Book b2 = new Book("George Orwell", "Nineteen Eighty-Four", 1949);

        if (b1 == b2) {
            System.out.print("equal, ");
        } else {
            System.out.print("not equal, ");
        }

        if (b1.equals(b2)) {
            System.out.print("equal");
        } else {
            System.out.print("not equal");
        }
    }
}

class Book {
    private String title;
    private String author;
    private int year;

    public Book(String a, String t, int y) {
        author = a;
        title = t;
        setYear(y);
    }

    public String toString() {
        return title + "\n\t" + author + "\n\t" + year;
    }

    public int getYear() {
        return year;
    }

    public void setYear(int y) {
        if (y >= 1450) {
            year = y;
        } else {
            year = 1900;
        }
    }

    public boolean equals(Book other) {
        if (this.title.equals(other.title) && this.author.equals(other.author)
                && this.year == other.year) {
            return true;
        }

        return false;
    }
}