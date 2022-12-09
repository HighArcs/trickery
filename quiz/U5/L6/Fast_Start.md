1. The `______` methods are used to change variables in classes, while the `______` methods are used to return the data. 

> mutator, accessor

2. Consider the following instance variables and method from a class:

```java
private String descriptor;

public String makeSentence(int quant, boolean sign) {
  String s = "Object is ";
  if (!sign) {
    s += "not ";
  }

  for (int i = 0; i < quant; i++) {
    s += "very ";
  }

  return s + descriptor;
}
```

Suppose an object of this class is initialized so the value of descriptor is "regular". What would be returned by the call makeSentence(3, false) made on this object?

> Object is not very very very regular

3. The class `Thingy` is defined as shown below.

```java
public class Thingy {
  private int stat;  

  public Thingy(int s) {
    stat = s;
  }

  public int getStat() {
    return stat;
  }

  public void setStat(int s) {
    stat = s;
  }
}
```

The code below appears in the main method of another class. What is printed when it is run?

```java
Thingy t = new Thingy(13);
t.setStat(7);
System.out.println(t.getStat());
```

> 7