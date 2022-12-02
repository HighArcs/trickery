1. Assuming that `scan` is a properly initialized `Scanner` variable, which of the following correctly inputs a double?

> `double val = scan.nextDouble();`

2. Consider the following code: 

```java
int x = -5;
x++;
System.out.println(x);
```

What is output?

> -4

3. Which of the following data types would be most appropriate to use when recording the answer to a yes or no question?

> boolean

4. Which of the following is a primitive data type?

> All of the above are primitive data types

5. What is output by the following code?

```java
int a = 11;
System.out.println(a / 2);
```

> 5

6. Which of the following is a legal variable name in Java?

> `nameOne`

7. What is `(19 % 7) * 2`?

> `10`

8. Which of the following would properly create A and B as double variables?

> ```java
> double A;
> double B;
> ```

9. Which of the following correctly stores the word Ford in a variable called car?

> `String car = "Ford";`

10. Consider the following code:

```java
int c = 10 – 55 % 2; 
System.out.println(c);
```

What is output?

> 9

11. Consider the following code: 

```java
int x = 10;
int y = 3;
System.out.println((x * y) / x );
```

What is output?

> 3

12. Consider the following variable declaration:

```java
double number = 23;
```

Does a cast need to be added so this code will compile and run successfully? `______`. If so, what should be typed for this cast? `_______`

> no, nothing

13. For which of the following would modular division be MOST likely to be useful?

> all of the above

14. The following code is intended to input two integers and print the average. What is a potential problem with the code as written? 

```java
System.out.println("Please enter two integers: ");
int a = scan.nextInt();
int b = scan.nextInt();
System.out.println("The average is: " + (a + b) / 2);
```

>  It needs a cast so that the decimal portion will be shown.

15. What is output by the following code?

```java
int num = 0;
num++;
num++;
num++;
num++;
num--;
num++;
num--;
num--;
num++;
System.out.println(num);
```

> 3

16. What is the value of num after executing this segment of code?

```java
int num = 21;
num += 72;
```

> 93

17. When might you encounter a problem with integer overflow? 

>  When trying to store an integer which is too big to be stored in an int variable

18. There are two integer variables in our program, hours and days, which represents time. If in the program, we increase the number of hours by one, which of the following lines of code will correctly update days and hours?

> ```java
> days = days + hours / 24; 
> hours = hours % 24;
> ```

19. Correct the following code so that p stores the nearest integer below 43.92.

```java
int p = 43.92;
```

> `int p = (int) 43.92;`

20. Which of the following will print the tens column of an integer stored in x? 

> `System.out.print(x / 10 % 10);`