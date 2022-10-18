1. What are if statements used for in programs?
> Controlling the sequence of execution 

2. The following code is intended to test whether the int variable num is less than 10, and if so print a warning and set it to 10. The code then prints the value of num.

```java
if (num < 10)
  System.out.println( "This is smaller than 10" );
  num = 10;
System.out.println("Value of num is " + num);
```

What correction should be made so the code functions as intended?

> Curly braces `{ }` should be added to enclose the second and third lines 

3. What is output?

```java
int a = 7;
int b = 8;
if (a + b <= 15) {
  System.out.println("The answer is: " + a + b);
}
```

> The answer is: 78

4. What is output to the screen by the following code?

```java
int x = 29 % 10;
if (x > 10) {
  System.out.println(1);
} else if (x > 8) {
  System.out.println(2);
} else if (x > 6) {
  System.out.println(3);
} else if (x > 4) {
  System.out.println(4);
} else {
  System.out.println(5);
}
```

> 2

5. Which of the following is a legal variable name in Java?

> other_var 

6. Which of the following code segments will print “large enough” when the square of the number var is larger than 25?

> ```java
> if (Math.pow(var, 2) > 25) {
>  System.out.println("large enough");
> }
> ```

7. Consider the following code:

```java
int diff = 0;
if (Math.abs(num1 - num2) == (num1 - num2)) {
  diff = num1 - num2;
} else if (Math.abs(num2 - num1) == (num2 - num1)) {
  diff = num2 - num1;
}
```

Which of the following will have the exact same result?

I. `int diff = Math.abs(num1) - num2;`

II. `int diff = Math.abs(num1 - num2);`

III. `int diff = Math.abs(num2 - num1);`

> II and III only 

8. Of the following code blocks, which one correctly executes exactly two commands when the condition is true?

I.

```java
if (y == 99)  {
  System.out.println("A");
  System.out.println("B");
}
System.out.println("C");
```

II.

```java
if (y == 99)
  System.out.println("A");
  System.out.println("B");
System.out.println("C");
```

III.

```java
if (y == 99)  {
  System.out.println("A");
  System.out.println("B");
}
```

> III only 

9. What is wrong with the following code?

```java
if (x = 7)
  System.out.print("It\'s true");
```

> The equals sign, =, should be replaced with a double equals, ==. 

10. What is output by the following code segment?

```java
int x = 11;
int y = 11;

if (x != y) {
  System.out.print("one");
} else if (x > y) {
  System.out.print("two");
} else if (y < x) {
  System.out.print("three");
} else if (y >= x) {
  System.out.print("four");
} else {
  System.out.print("five");
}
```

> four