1. What will be printed when the following code is run?

```java
int a = 4;
int b = 3;
if (a < 5 || a % b == 0) {
  System.out.print("yes");
} else {
  System.out.print("no");
}
```

> yes

2. What will be printed when the following code is run?

```java
int x = 12;
int y = 17;
if (x + y < 30 && x < y) {
  System.out.print("one");
} if (!(x == 20) || y == 17) {
  System.out.print("two");
}
```

> onetwo

3. What value is printed by the following code?

```java
int a = 1;
int b = 2;

if (a <= 2 && b < a) {
  a *= 2;
} else {
  b *= 2;
}

if (!(a == 3 || b == 4)) {
  a *= 3;
} else {
  b *= 5;
}

System.out.println(a + " " + b);
```

> 1 20

4. Which is not a correct boolean condition?

A. `(x < y && 7 == 9)`
B. `(y ! >= 9 || x == 9)`
C. `(x == 9 || y != 9)`
D. `(!(x > y) && y == 9)`

> B

5. The following truth table matches which boolean condition?

| A   | B   | ___ |
| --- | --- | --- |
| 1   | 1   | 0   |
| 1   | 0   | 1   |
| 0   | 1   | 1   |
| 0   | 0   | 1   |

> !A || !B

6. The following truth table matches which boolean condition?

| A   | B   | ___ |
| --- | --- | --- |
| 1   | 1   | 1   |
| 1   | 0   | 1   |
| 0   | 1   | 1   |
| 0   | 0   | 0   |

> A || (A || B)

