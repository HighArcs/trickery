1. A single basic array can hold data of multiple different types. 

> False

2. Basic arrays cannot be resized. 

> True

3. The following array has been declared:

```java
int[] a = { 30, 32, 17, 25, 33, 16, 27, 29, 27, 27, 13, 15, 35, 36, 19, 18, 28 };
```

The indexes of this array start at `______` and go to `______`.

> 0, 16

4. Which of the following code segments create an array containing the integers 10 to 13 inclusive? Select all that apply.

> `int[] a = {10, 11, 12, 13};`
>
> ```java
> int[] a = new int[4];
> a[0] = 10;
> a[1] = 11;
> a[2] = 12;
> a[3] = 13;
> ```

5. Assume the following code:

```java
int[] a = { 1, 2, 3, 4, 5, 6, 7, 8, 9 };
a[5] = a[5] + a[1];
```

What is the value of `a[5]` after this code has been executed?

> 8

6. Assume the following code:

```java
int[] a = { 1, 2, 3, 4, 5, 6, 7, 8, 9 };
a[8]--;
```

What is the value of `a[8]` after this code has been executed?

> 8

7. Assume the following code:

```java
int[] a = { 1, 2, 3, 4, 5, 6, 7, 8, 9 };
a[6] = a[4] / 2;
```

What is the value of `a[6]` after this code has been executed?

> 2

8. After the following code has been executed, what values will the array a hold?

```java
int[] a = { 1, 2, 3, 4, 5, 6, 7 };
a[3]++;
a[2] = a[3] + 1;
a[6]--; 
a[5] = a[2] / 2;
```

> `1 2 6 5 5 3 6`