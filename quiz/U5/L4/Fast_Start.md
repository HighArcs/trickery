1. Consider the following method declaration.

```java
public static void reset(Rectangle r, Circle c) {
  r = new Rectangle(1.0, 1.0);
  c.setRadius(1.0);
}
```

If the following code in the main method of the program is run, what will be printed?

```java
Rectangle rect = new Rectangle(5, 7);
Circle circ = new Circle(9.0);
reset(rect, circ);
System.out.println(rect);
System.out.println(circ);
```

> rectangle with length 5.0, width 7.0
> 
> circle with radius 1.0

2. Consider the following code:

```java
int b = -3;
int e = 2;

int x = Math.abs((int) Math.pow(b, e));
if (x == 4) {
  System.out.print("first");
} else if (x == 9) {
  System.out.print("second");
} else if (x == -9) {
  System.out.print("third");
} else if (x == -9) {
  System.out.print("fourth");
} else {
  System.out.print("fifth");
}
```

What will be printed?

> second

3. Given the following method definition:

```java
public static void doStuff(int a) {
  a++;
}
```

What is output by the following?

```java
int x = 9;
doStuff(x);
System.out.println(x);
```

> 9