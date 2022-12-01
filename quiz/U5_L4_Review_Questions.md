1. When a method needs to send a value back to a calling method, it uses the `______` command. 

> return

2. Which of the following is true about methods? 

> A method in Java can only return one value. 

3. Suppose a method p has the following heading:

```java
public static Circle p()
```

Which return statement may be used in `p()`?

> `return new Circle(6.5);`

4. Consider the following method:

```java
public static Circle doubleRad(Circle c) {
  c.setRadius(2 * c.getRadius());
  return c;
}
```

What will be printed when the following code in the main method of the program is run?

```java
Circle circ1 = new Circle(3.0);
Circle circ2 = doubleRad(circ1);
System.out.println(circ1);
System.out.println(circ2);
```

> circle with radius 6.0
> 
> circle with radius 6.0

5. Consider the following method:

```java
public static int doubleVal(int n) {
  n *= 2;
  return n;
}
```

What will be printed when the following code in the main method of the program is run?

```java
int a = 5;
int b = doubleVal(a);
System.out.print(a + " " + b);
```

> 5 10
