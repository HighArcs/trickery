1. When is the initialization statement (shown in bold below) run in a for loop?

```java
for (int i = 0; i < 10; i++)
```

> Before the first run of the loop only, before the boolean condition is checked.

2. When is the increment statement (shown in bold below) run in a for loop?

```java
for (int i = 0; i < 10; i++)
```
> At the end of every run of the loop, before the boolean expression is checked for the next iteration of the loop.

3. Consider the following code:

```java
String w = "onomatopoeia";
for (int i = 0; i < w.length(); i++) {
  System.out.print(w.substring(i, i + 1) + "  ");
  if (i % 3 == 2) {
     System.out.println();
  }
}
```

What is output?

> o  n  o
> m  a  t
> o  p  o
> e  i  a