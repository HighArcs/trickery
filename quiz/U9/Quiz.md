1. All classes in Java inherit from the `______` class. 

> Object

2. Which of the following keywords allows a child class to access the overridden methods in a parent class? 

> `super`

Problems 3 - 4 refer to the following class meant to represent the spinner in a board game:

```java
class Spinner {
  private int numOptions; 
  private int chosen;

  public Spinner(int n) {
    if (n >= 2 && n <= 20) {
      numOptions = n;
    }

    spin();
  }

  public void spin() {
    chosen = (int) (Math.random () * numOptions) + 1;
  }
}
```

3. Which of the following is a correct declaration for a Spinner object as written?

> `Spinner s = new Spinner(15);`

4. Which of the following would be correct declarations for a default constructor?

I. 	
```java
public Spinner() {
  this(2);
}
```

II. 	
```java
public Spinner() {
  numOptions = n;
}
```

III. 	
```java
public Spinner() {
  numOptions = 2;
  spin();
}
```

> I and III only

5. Which of the following methods is from the Object class and often overridden? 

> `equals`

6. Which of the following is true about parent and child classes?

> A child class can access the parent's constructor using the super keyword.

7. You have written a class called Frog and have not written a toString method.

```java
Frog f = new Frog();
System.out.println(f);
```

What happens when the code above is executed?

> The `toString` method in `Object` is called.

8. What is the rule for a super reference in a constructor? 

> It must be the first line of the constructor in the child class.

9. Consider the following class definition.

```java
public class WhatsIt {
  private int length;
  private int width;

  public int getArea() {
    // implementation not shown
  }

  private int getPerimeter() {
    // implementation not shown
  }
}
```

A child class `Thingy` that extends `WhatsIt` would have access to:

> `getArea()`

10. Consider the following class:
```java
public class FrozenDesert {
  public FrozenDesert() {
    System.out.println("Yum"); 
  }
}
```

You write a class, FrozenYogurt, which extends FrozenDesert. Which of the following is a correct implementation of the constructor for FrozenYogurt?

I. 	
```java
public FrozenYogurt() {
  System.out.println("I'm the new ice cream");
  super();
}
```

II. 	
```java
public FrozenYogurt() {
  super();
  System.out.println("I'm the new ice cream");
  super();
}
```

III. 	
```java
public FrozenYogurt() {
  super();
  System.out.println("I'm the new ice cream");
}
```

> III only