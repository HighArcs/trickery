1. Consider the following code:

```java
int x = 5;
while (x < 9) {
    System.out.print(x + " ");
    x += 2;
}
```

What is output?

> 5 7

2. What is output by the following code?

```java
int a = 5;
int s = 0;
while (a % 4 != 0) {
  s += a;
  a++;
}

System.out.println(s);
```

> 18

3. What is output by the following code?

```java
String str = "processor";
System.out.print(str.substring(str.length() - 1));
System.out.print(str.substring(1, 2));
```

> rr