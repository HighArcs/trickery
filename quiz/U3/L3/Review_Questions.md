1. What is printed to the screen after the following code runs?

```java
int x = 21;
if (x > 21) {
  System.out.println(1);
} else if (x < 21) {
  System.out.println(2);
} else {
  System.out.println(3);
}
```

> 3

2. When do you use an `else` statement? 

> To run some code when an if statement is false

3. What is printed to the screen after the following code runs?

```java
int x = 8 % 2;
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

> 5

4. The program below is intended to print a grade given a mark input as shown in the table:

| Mark         | Grade |
| ------------ | ----- |
| 90 and above | A     |
| 80 to 89     | B     |
| 70 to 79     | C     |
| 60 to 69     | D     |
| 59 and below | F     |

Which commands correctly fill in the two blanks in the code?

```java
if (mark >= 90) {
  System.out.println("A");
} else if (mark >= 80) {
  System.out.println("B");
} ______ (mark >= 70) {
  System.out.println("C");
} ______ (mark >= 60) {
  System.out.println("D");
} else {
  System.out.println("F");
}
```

> `else if`, `else if`

5. Consider the following code.

```java
int a = 10;

if (a > 7) {
  System.out.print("one");
} else if (a % 5 == 0) {
  System.out.print("two");
} else {
  System.out.print("three");
} if (a * 2 >= 18) {
  System.out.print("four");
}
```

What is output when this code is executed?

> onefour