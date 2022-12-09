1. Which of the following can be used to show the start of a comment in Java?

There can be multiple answers to this question.

> /* 
>
> //

2. Which of the following are usually made public in a class?

There can be multiple answers to this question.

> Accessor methods
>
> Constructors
>
> Mutator methods

3. The class Purchase is defined as below

```java
public class Purchase {
  private int purchaseValue;
  private String customerName;
  private boolean paid;

  public Purchase(int p) {
    this(p, "customer");
  }

  public Purchase(String n) {
    this(100, n);
  }
  
  public Purchase(int p, String n) {
    purchaseValue = p;
    customerName = n;
    paid = false;
  }

  public String toString() {
    String s = "$" + purchaseValue + " from " + customerName + " - ";

    if (!paid) {
      s += "not ";
    }

    return s + "paid"; 
  }
}
```

What is printed by the following code which appears in the main method of another class?

```java
Purchase x = new Purchase("gencorp");
System.out.print(x);
```

> $100 from gencorp - not paid