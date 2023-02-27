1. Which of the following is false?

> It is possible for a child class to have more than one parent class.

2. Assume you are writing a class `Elephant`, which inherits from a parent class, `Animal`. A partial declaration of the class `Animal` is below.

```java
public class Animal {
    public int limbs;
    public String species;
    
    public Animal(int limbsAmt, String name) {
        limbs = limbsAmt;
        species = name;
    }
}
```

Which of the following code segments correctly implements the constructor `Elephant()`?

I.
```java
public Elephant(int limbsAmt, String name) {
    limbs = limbsAmt;
    super(limbs, "Elephant");
}
```

II.
```java
public Elephant(int limbsAmt, String name) {
    limbs = 4;
    species = name;
}
```

III.
```java
public Elephant() {
    super(4,"Elephant");
}
```

> III

3. A child class can call all of its parent's public methods

> True