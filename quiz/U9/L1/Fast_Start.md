1. A constructor cannot initialize a private value

> False

2. A(n) `______` method which can only be called after the class it belongs to is declared.

> instance

3. Consider the following Home class;

```java
public class Home { 
    private String color;
    private int size;

    /* constructors not shown */
    public void setColor(String c) {
        /* implementation not shown */
    }

    public void setSize(int s) {
        /* implementation not shown */
    }
}
```

Which of the following constructors for the Home class would not be legal?

> ```java
> public Home(String c) {
>     color = c;
>     size = 4;
>     return color;
> }
> ```


