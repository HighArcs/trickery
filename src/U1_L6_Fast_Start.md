1. Which of the following expressions in Java is equal to 1? 

> `7 % 3`

2. What is printed when the following code has been executed?

```java
int x = 13;
int y = 6;
System.out.println((2 * x + y) % x);
```

> 6

3. Consider the following program which is intended to get the amount of time (in seconds) it took someone to do one of their chores and then converts it to minutes and seconds.

```java
Scanner scan = new Scanner(System.in);
int m = 0;
int s;
System.out.print("How many seconds did it take you to do one of your chores? ");
s = scan.nextInt();
/* missing code */
System.out.println("It took you " + m + " minutes and " + s + " seconds.");
```

Which of the following could replace the missing code so that it all works as intended?

> ```java
> m = s / 60;
> s = s % 60;
> ```