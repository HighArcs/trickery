1. Which of the following is NOT true about constructors? 

> Constructors are declared using the void keyword.

2. The `______` keyword creates an object in memory and assigns the memory reference to the variable. 

> new



Questions 3-5 refer to the following code:

```java
public class Box {
  private int length;
  private int height;
  private int width;
 
  public Box() {
    length = 1;
    height = 1;
    width = 1;
  }

  public Box(int l, int w, int h) {
    this();
    if (l > 0) {
      length = l;
    }

    if (h > 0) {
      height = h;
    }

    if (w > 0) {
      width = w;
    }
  }
 
  public int getVolume() {
    return height * length * width;
  }
 
  public String toString() {
    String t = "length: " + length;
    t += "\nwidth: " + width;
    t += "\nheight: " + height;
    t += "\nvolume: " + getVolume();
    return t;
  }
}
```

3. What is output by the following code?

```java
Box b2 = new Box(3, 3, 3);
System.out.println(b2.getVolume());
```

> 27

4. What is output by the following code?

```java
int total = 0;
for (int i = 3; i < 6; i++) {
  Box b = new Box(i, 3, 1);
  total += b.getVolume();
}
System.out.println(total);
```

> 36

5. Consider the following code:

```java
Box b1 = new Box(3, 5, 2);
Box b2 = new Box();
Box b3 = new Box(3, 2, 2);
```

The line, `Box b2 = new Box()`; calls the `______` of Box.

> default constructor

6. Consider the following class and its constructors:

```java
public class Bus {
  private int number;
  private String destination;

  public Bus() {
    number = 1;
    destination = "unknown";
  }

  public Bus(int n) {
    this();
    if (n > 0) {
      number = n;
    }
  }
  
  public Bus(int n, String d) {
    this(n);
    destination = d;
  }

  public String toString () {
    return "number " + number + " bus to " + destination;
  }
}
```

What is printed when the following code, in the main method of a separate class, is run?

```java
Bus b = new Bus(-2, "hospital")
System.out.println(b);
```

> number 1 bus to hospital