1. Consider the following code:

```java
int count = 4;
while (count <= 7) {
  count++;
  System.out.print(count + " ");
}
```

What are the first and last numbers output?

> 5 8

2. When would it be more beneficial to use a while loop instead of a for loop? 

> When you need your loop to be controlled by user input.

3. Choose which three values should replace the blanks in the for loop so that it loops through the numbers: 3 6 9 12 15. 

Note that the choices will fill in the three blanks in the order which they appear.

```java
for (int i = ______ ; i ______ ; i ______ ) {
  System.out.print(i + " "); 
}
```

> 3, <= 15, += 3

4. How many times will the following loop repeat?

```java
int num = 49;
while (num > 0) { 
  if (num % 2 == 0) {
    num++;
  } else {
    num--;
  }
}
```

> Infinite Loop 

5. The following loop is intended to print the even numbers from 20 to 26 inclusive:

```java
int x = 20;
while (x < 26) {
  System.out.print(x);
  x++;
}
```

What would you need to change in order for the code to work correctly?

> The x++ needs to be x += 2 and the x < 26 needs to be <=

6. What is output to the screen by the following code?

```java
int num = 1987;
while (num > 0) {
  num = num / 10;
  System.out.print(num % 10 + " ");
}
```

> 8 9 1 0

7. Consider the following code segment:

```java
int c = 1;
while (c <= 10) {
  if (c % 3 == 1)
    System.out.print(c + " ");
  c++;
}
```

Which of the following produce the exact same output?

I.
```java
int c = 1;
while (c <= 10) {
  c++;
  if (c % 3 == 1)
    System.out.print(c + " ");
}
```
 

II.
```java
int c = 1;
while (c <= 10) {
  System.out.print(c + " ");
  c += 3;
}
```

III.
```java
int c = 0;
while (c <= 10) {
  c++;
  if ( c % 3 == 1)
    System.out.print(c + " ");
}
```

> II and III only

8. Choose which three values should replace the blanks in the for loop so that it loops through the numbers: 10 9 8 7 6 5 4 3 2 1. 

Note that the choices will fill in the three blanks in the order which they appear.

```java
for (int i = ______; i ______; i ______) {
  System.out.print(i + " ");
}
```

> 10, >= 1, --

9. Consider the following code segment.

```java
int a = 0;
while (/* missing code */) {
  System.out.print((a + 5) + " ");
  a += 10;
}
```

I. `a < 45`

II. `a < 40`

III. `a != 45`

Which of the proposed replacements for `/* missing code */` will cause the code segment to print only the values 5 15 25 35?

Consider the following possible replacements for `/* missing code */`.

> II only

10. Consider the following code segment.

```java
for (int k = 30; k > 0; k = k - 3) {
  if (k % 5 == 0) {
    System.out.print(k + " ");
  }
}
```

What is printed as a result of executing the code segment?

> 30 15s