1. What will the output be?

```java
int x = 7;
int y = 7;
if (x < y) {
  System.out.println("x is less than y");
} else {
  System.out.println("x is not less than y");
}
```

> x is not less than y

2. Under what conditions will "correct!" be printed?

```java
if (weight <= 10) {
  if (weight > 3) {
    System.out.println("correct!");
  }
}
```

> When `weight` is less than or equal to `10` and greater than `3`. 

3. What number is printed by the following code segment?

```java
int a = 13;

if (a % 3 == 0) {
  a++;
} else if (a <= 13) {
  a += 3;
} else {
  a += 5;
}

System.out.println(a);
```

> 16