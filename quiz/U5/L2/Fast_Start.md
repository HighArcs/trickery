1. Consider the following java program.

```java
class Example {
  public static void myMethod() {
    System.out.print("hello ");
  }

  public static void main(String[] args) {
    myMethod();
    myMethod();
  }
}
```

What is printed when this program runs?

> `hello hello`

2. What is wrong with the following code which uses the circle class from the shapes package?

```java
Circle c = new Circle();
double r = c.setRadius(6.5);
```

> The `setRadius` method is void, so it doesn't have a return to be stored in a value. 

3. Consider the following code:

```java
class Example {
  public static void main(String[] args) {
    doStuff();
    doStuff();
    
    public static void doStuff() {
      System.out.println("You called?");
    }
  }
}
```

Why does this cause an error when compiled?

> The method `doStuff` can't be defined inside of the main method. 