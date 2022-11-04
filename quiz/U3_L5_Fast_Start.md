1. What is printed by the following code?

```java
int x = 2;
int y = 4;
if (x == 2 && y == 2) {
  System.out.println("egg");
} else if (x == 2 || y == 2) {
  System.out.println("bacon");
} else {
  System.out.println("sausage");
}
```

> bacon

2. What is printed by the following code segment?

```java
boolean a = true;
boolean b = false;
boolean c = true;

if (!(a || b) || c) {
  System.out.print("W");
} else {
  System.out.print("X");
}

if ((a && b) || (!b && c)) {
  System.out.print("Y");
} else {
  System.out.print("Z");
}
```

> WY

3. Given the variable declarations:

```java
int x = 24;
int y = 53;
```

> `!(x <= y && x == y)`
>
> `x >= y || x != y`

Which of the following boolean statements evaluate to true? Choose all correct answers.