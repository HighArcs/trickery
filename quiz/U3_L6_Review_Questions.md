1. Assume grade is an int variable. Which of the following is equivalent to `!(grade > 90)`? 

> `grade <= 90`

2. Of the following choices, which expression is `!(x > y || z == 9)` equivalent to?

> `x <= y && z != 9`

3. Which of the following statements is shown in the final column of the truth table?

| A   | B   | ___ |
| --- | --- | --- |
| 0   | 0   | 1   |
| 0   | 1   | 1   |
| 1   | 0   | 0   |
| 1   | 1   | 1   |

> `!(A && !B)`

4. What is printed by the following code?

```java
x = 5;
if (!(x > 5 || x <= 2)) {
  System.out.println("in range 1");
} else {
  System.out.println("not in range 1");
}

if (!(x >= 4 && x < 7)) {
  System.out.println("in range 2");
} else {
  System.out.println("not in range 2");
}
```

> ```
> in range 1
> not in range 2
> ```