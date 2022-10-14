1. When does the following Boolean condition evaluate to true?

```java
!(x > y || x == y)
```

> When x is less than y.

2. Which of the following is equivalent to `x > 17 && x <= 21`?

> `!(x <= 17 || x > 21)`

3. What is printed by the following code?

```java
int x = 15;

if (!(x >= 0 && x % 10 != 0)) {
    System.out.println("Condition 1");
} else if (!(x % 5 != 0 || x % 3 != 0)) {
    System.out.println("Condition 2");
} else {
    System.out.println("Condition 3");
}
```

> Condition 2