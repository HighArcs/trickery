1. For the following array, what is the maximum number of comparisons a binary search will need to do?
```java
int[] a = { 2, 8, 59, 20, 128, 3, 6, 3, 7, 1 };
```

> 4

2. Consider the following recursive method.
```java
public static void recur(int n) {
    if (n % 5 != 0) {
        recur(n - 3);
    }

    System.out.print(n / 5);
}
```

What is printed by the call `recur(63)`?

> 1212

3. Consider the following recursive method.
```java
public static String recur(String s) {
    if (s.length() <= 1) {
        return "";
    }

    return recur(s.substring(2)) + s.substring(1,2);
}
```


What is returned by the call `recur("javaiscool")`?

> losaa