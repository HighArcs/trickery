1. When a primitive type parameter is passed to a method, the method makes a copy of the value and stores it as a `_____` variable. 

> local

2. Consider the following recursive method:
```java
public static void recur1(int n) {
    if(n > 0) {
        recur1(n - 2);
    }

    System.out.print(n + " ");
}
```

What is output by the call, `recur1(5)`?

> `-1 1 3 5`

3. Which of the following outputs "available" if x equals 10 and outputs "unavailable" if it is not? 

> ```java
> if(x == 10) {
>   System.out.print("available");
> } else {
>   System.out.print("unavailable");
> }
> ```


