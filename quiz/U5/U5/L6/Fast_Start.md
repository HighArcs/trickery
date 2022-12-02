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