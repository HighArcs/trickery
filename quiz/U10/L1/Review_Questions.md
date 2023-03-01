1. A recursive method's continuous recursive calls are generally stopped using a `_____`. 

> base case

2. Which of the following methods is recursive?

I. 	
```java
public static int mysteryI(int n) {
    if(n > 10) {
        return n - 5;
    }
}
```

II. 	
```java
public static int mysteryII(int n) {
    if(n > 10) {
        mysteryII(n / 10);
    }

    return n - 2;
}
```

III. 	
```java
public static int mysteryIII(int n) {
    if(n > 0) {
        n--;
        mysteryIII(n);
    }

    return n - 2;
}
```

> II and III

3. Under what conditions will the following recursive method NOT stop repeating? (Infinite recursion)

```java
public static void mystery(int n) {
    if(n <= 0) {
        System.out.println("Done!");
    } else {
        mystery(n+1);
    }
}
```

> When n is greater than zero

4. In the following recursive method on what line number is the recursive call?

```java
1 | public static void mystery(int n) {
2 |     if(n <= 0) {
3 |         System.out.println(n % 3);
4 |     } else {
5 |         mystery(n / 2);
6 |     }
7 | }
```

> `5`

5. Consider the code:
```java
public static void mystery(int n) {
    if(n > 0) {
        n--;
        mystery(n);
    } 

    System.out.print((n + 1) + " ");
}
```

What is output by the following call?
```java
mystery(9);
```
\- **note:** this is the output given by running the code directly. none of the answer choices reflect this.
 
> 1 1 2 3 4 5 6 7 8 9