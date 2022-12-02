1. What mistake is in the following code?

```java
public static void mystery(double a) { 
  System.out.println(a * 3.14); 
  return a * 3.14;
}
```

> A method whose return type is `void` cannot return a value. 

2. Consider a method defined with the header:

```java
public static void doStuff(double a, int b)
```

Which of the following method calls is **not** legal?

> doStuff(5, 7.5);

3. When you pass a double variable to a method, the method receives `______`.

> A copy of the variable

4. Consider the following method:

```java
public static boolean tryIt(int a, int b) {
  if ((a - b) == Math.abs(a - b)) {
    return true;
  }
  return false;
}
```

Which of the following methods produce the same results?

> III only

5. Which of the following is true about overloaded methods?

> Java cannot use a method's return type to tell two overloaded methods apart. 

6. Given the following code, what is output by the method call, `mystery(5, 7.0015)`?

```java
public static void mystery(int a) {
  System.out.println("A");
}

public static void mystery(double a) {
  System.out.println("B");
}

public static void mystery(int a, double b) {
  System.out.println("C");
}

public static void mystery(double a, int b) {
  System.out.println("D");
}
```

> C

7. Consider the following method which makes use of the shapes.Rectangle class.

```java
public static void stretch(Rectangle r) {
  r.setLength(2 * r.getLength());
  System.out.print(r.getLength());
}
```

What is printed when the following code in the main method is executed?

```java
Rectangle rect = new Rectangle(10, 6);
stretch(rect);
System.out.print(" " + rect.getLength());
```

> 20.0 20.0

8. Consider the following methods:

```java
public static double doStuff(int a) {
  return a / 2;
}

public static double doStuff(double val) {
  return val / 10;
}
```

What is output by the following?

```java
System.out.println(doStuff(5) + doStuff(5.0));
```

> 2.5

9. What is output by the following code?

```java
public static void stuff(int w) {
  w -= 2;
}

public static void main(String[] args) {
  int n = 2;
  stuff(n);
  System.out.print(n);
}
```

> 2

10. Consider the following methods:

```java
public static void printSport(double n) {
  System.out.print("football ");
  printSport((int) n);
}

public static void printSport(int n) {
  System.out.print("basketball ");
}
```

What is output by the method call `printSport(8)`?

> basketball