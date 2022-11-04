1. What is printed by the following code?

```java
int x = 13;
x++;

if (x == 14) {
  System.out.println("OK");
}

if (x == 16) {
  System.out.println("Let's go!");
}
```

> OK

2. What is wrong with the following code?

```java
Scanner scan = new Scanner(System.in);
int x = scan.nextInt();
if (x = 9) {
  System.out.println(x);
}
```

> = is incorrect, it should be == 

3. Consider the following code segment.

```java
int r = (int) (10 * Math.random()) + 1;
if (r == 1) {
  System.out.println("bullseye");
}
```

If I run this code segment lots of times, what do you expect to happen?

> "bullseye" will be printed occasionally 

4. What does the following if statement do?

```java
if (num1 == Math.abs(num1))
```

> Tests if the value in num1 is greater than or equal to zero. 
