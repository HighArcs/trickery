1. What is output by the following code?

```java
int a = 0;
for (int i = 1; i < 5; i++) {
  for (int j = 1; j < 4; j++) {
    a++;
  }
}
System.out.println(a);
```

> 12

2. What is output by the following code?

```java
for (int i = 1; i < 4; i++) {
  for (int j = 0; j < i; j++) {
    System.out.print(i);
  }
  System.out.println();
}
```

> 1
> 
> 22
> 
> 333

3. What is output by the following code?

```java
for (int i = 5; i >= 2; i--) {
  for (int j = 2; j <= 4; j++) {
    System.out.print(i * j + " ");
  }
  System.out.println();
}
```

> 10 15 20
> 
> 8 12 16
> 
> 6 9 12
> 
> 4 6 8

4. What is output by the following code?

```java
for (int i = 0; i < 4; i++) {
  for (int j = i; j < 5; j++) {
    System.out.print(j + " ");
  }
  System.out.println();
}
```

> 0 1 2 3 4 
> 
> 1 2 3 4
> 
> 2 3 4
> 
> 3 4

5. What is output by the following code?

```java
String str = "RAM";
for (int i = str.length(); i > 0; i--) {
  for (int j = 1; j <= 3; j++) {
    System.out.print(str.substring(i - 1, i));
  }
}
```

> MMMAAARRR

6. What is output by the following code?

```java
int x = 0;
while (x < 5) {
  int y = 5;
  while (x < y) {
    y--;
    x++;
  }
  System.out.print(y + " ");
}
```

> 2 4 4