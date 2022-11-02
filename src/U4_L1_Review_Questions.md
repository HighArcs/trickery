1. Consider the following code:

```java
int a = 0;
int b = 0;
while (a < 5 && b < 3) {
  System.out.println(a + " " + b);
  a++;
  b++;
}
```

What is output?

> ```
> 0 0
> 1 1 
> 2 2 
> ```

2. Consider the following code:

```java
int n = 14;
while (!(n % 3 == 0 && n % 5 == 0)) {
  System.out.println(n);
  n += 2;
}
```

What is the last number printed by this code?

> 28

3. What is one potential problem with the following loop?

```java
System.out.print("Enter integers. Enter -1 to exit.");
System.out.println(" Count of integers entered will be returned.");

int n = 0;
int c = 0;

while (n != -1) {
  n = scan.nextInt();
  c++;
}

System.out.println(c);
```

> The loop counts when the user enters -1 into the keyboard, so the count will be one too many.

4. What is one potential problem with the following loop?

```java
int n = 5;
while (n != -1) {
  System.out.println(n);
}
```

> Since n does not change the loop will not stop. 

5. Consider the following code:

```java
int n = 4;
while (n <= 15)  {
  n += 2;
  System.out.print(n + " ");
}
```

What is output?

> 6 8 10 12 14 16

6. Consider the following code:

```java
int n = 80;
while (n > 40) {
  System.out.print(n % 10 + " ");
  n -= 5;
}
```

What is output?

> 0 5 0 5 0 5 0 5