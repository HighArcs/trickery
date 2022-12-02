1. Classes include variables and `______`. 

> methods

2. All instance variables should be declared as `_______`

> private

3. The following four questions refer to the Book class defined below:

```java
public class Book {
  private String title;
  private String author;
  private int year;

  Book(/* constructor header not shown */) {
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
```

The header for the constructor of this class is missing. Which of the following would be the best header to use for this constructor?

> `public Book(String a, String t, int y)`

4. The following code appears in the main method of another class.

```java
Book b = new Book("Emily Bronte", "Wuthering Heights", 1847);
System.out.println(b);
```

Which of the following best describes what appears on the screen when this code is executed?

> The values "Wuthering Heights", "Emily Bronte" and "1847" with each on a new line, and the second two lines indented. 

5. The year variable for a Book is set to 2001. Suppose the variable b in a separate class points to this book. Which of the following calls would return the year value of 2001? 

> `b.getYear()`

6. What is printed when the following code, which appears in the main method of a separate class to Book, is run?

```java
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
```

> not equal, equal