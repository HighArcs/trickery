1. Which of the following code segments will produce the same output as the segment below?

```java
for (int i = 5; i < 14; i += 3) {
  System.out.print(i);
}
```

> ```java
> int i = 5;
> while (i < 14) {
>   System.out.print(i);
>   i += 3;
> }
> ```

2. Which of the following best describes what the following algorithm does? You may assume the variable s is initialized as a non-empty String.

```java
boolean r = true;
for (int i = 0; i < s.length() - 1; i++) {
  if (s.substring(i, i + 1).equals(s.substring(i + 1, i + 2))) {
    r = false;
  }
}

if (r) {
  System.out.println("pass");
}
```

> The algorithm prints "pass" if there are no pairs of consecutive letters in s which are the same.

