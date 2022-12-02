1. Short circuit evaluation means that in the code:

```java
if (x >= z || val == 97)
```

> if `x >= z` is true it doesn't evaluate `val == 97`

2. When does this evaluate to false regardless of the unknown code?

```java
if ( x < y && /*unknown code*/)
```

> When `x` is greater than or equal to `y`.

3. When does this evaluate to true regardless of the unknown code?

```java
if ( x < y || /*unknown code*/)
```

> When `x` is greater than or equal to `y`. 

4. What is printed when the following code is evaluated.

```java
int x = 12;
int y = 13;

if (x == y && (2 * x / 3 + 3 * y / 4 == x / 3 * 2 + y / 4 * 3)) {
  System.out.print("yes");
} else {
  System.out.print("no");
}
```

> no

5. What is printed when the following code is executed?

```java
int x = 17;
int y = 11;

if (y < x || ((7 * x - 6 * y) % x) == 1) {
  System.out.print("yes");
} else {
  System.out.print("no");
}
```

> yes

