1. Consider the following code:
```java
public static double mystery(int n) {
    if (n > 0) {
        return mystery(n - 2);
    }

    return n;
}
```

What is returned by the method call, `mystery(13);`?

> -1.0

2. Consider the following code:
```java
public static void mystery(String digits, int n) {
    System.out.println(digits); 
    digits = digits.substring(0, n);

    if (n > 0) {
        mystery(digits, n-1);
    }
}
```

How many total times will the method mystery be executed with the method call, mystery("0123456789", 5); INCLUDING the initial call to mystery?

> 6

3. Consider the following code:
```java
public static int mystery(int n) {
    n--;
    if (n > 0) {
        mystery(n);
    }

    return n + 2;
}
```

What is returned by the method call, `mystery(7);`?

> 8

4. Consider the following code:

```java
public static int mystery(int n) {
    n -= 7;
    if (n <= 0) {
        return n + 5;
    }

    mystery(n + 1);
}
```

Why won't it run?

> Because the only return statement is within an if statement. 

5. Consider the following code:

```java
public static int mystery(int a, int b) {
    System.out.println(a + " " + b);

    if (a > b) {
        return mystery(a, Math.abs(b) + 1);
    } else if (b > a) {
        return mystery(Math.abs(a) + 1, b);
    }

    return 1;
}
```

What is output with the method call, `mystery(5, 2);`?

> 5 2
>
> 5 3
> 
> 5 4 
>
> 5 5

6. Consider the following recursive method.
```java
public static String recur(String str) {
    int len = str.length();

    if(len >= 2) {
        String next = str.substring(0, len - 2);
        return recur(next) + str.substring(len - 2);
    } else {
        return str;
    }
}
```

What is returned by the call `recur("avocado")`?

> `avocado`
