1. Consider the following code segment in which str is a String with length greater than 1.

```java
for (int i = 0; i < str.length() - 1; i++) {
  if (str.substring(i, i + 2).equals("oh")) {
    System.out.println("Found it!");
  }
}
```

Which of the following best describes what this code segment does?

> Prints out "Found it!" for every substring which is equal to "oh".

2. The following code segment displays the number of times the character 'a' appears in the String str.

```java
int count = 0;
for (int i = 0; i < str.length(); i++) {
  String letter = str.substring(i, i + 1);
  if (letter.equals("a")) {
    count++;
  }
}
System.out.println(count);
```

Which of the following modifications to the code will ensure the number it displays includes times the capital 'A' appears?

>  Change the if statement condition to `letter.equals("a") || letter.equals("A")`.

3. What is printed by the following code segment?

```java
String str = "normality";
for (int i = str.length(); i > 1; i -= 2) {
  System.out.print(str.substring(i - 1, i));
}
```

> yiar

4. What is printed by the following code?

```java
String str = "an anaconda and an ant";
int count = 0;
for (int i = 0; i < str.length() - 1; i++) {
  if (str.substring(i, i + 2).equals("an")) {
    count++;
  }
}

System.out.println(count);
```

> 5