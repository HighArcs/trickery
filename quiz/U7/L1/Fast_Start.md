1. Consider the array declaration:

```java
int[] list = new int[40];
```

Which of the following correctly returns the length of the array?

> `list.length`

2. Consider the following code segment:

```java
int[] list = {1, 6, 23, 7, 3, 5, 6};
int t = 0;
for (int i = 0; i < list.length; i++) {
  t = t + list[i];
}
```

What is the value of t after this code segment has executed?

> 51

3. Consider the following code segment:

```java
int[] list = { 1, 6, 23, 7, 3, 5, 6 };
int t = 0;
for (int i = 0; i < list.length; i += 2) {
  t = t + list[i];
}
```

What is the value of t after this code segment has executed?

> 33