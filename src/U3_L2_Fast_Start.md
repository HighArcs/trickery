1. After the following code has been executed, will the if statement evaluate to true or false?

```java
int x = -6;
int y = 3;
if (x == y) {
  // other code
}
```

> False

2. What is output?

```java
double test = 76.5;
if (test == 76.5) {
  System.out.print("C");
}

if (test == 84.5) {
  System.out.print("B");
}

if (test == 92.5) {
  System.out.print("A");
}
```

> C

3. Consider the following code intended to test if a double entered from the keyboard is positive:

```java
double a = scan.nextInt();

if (a == Math.abs(a)) {
  System.out.println("positive number");
}
```

What is a potential mistake?

> The `scan.nextInt()` should use `scan.nextDouble()`