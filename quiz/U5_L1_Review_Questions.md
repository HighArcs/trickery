1. A `______` is a separate chunk of code that is given a name.

> method

2. When you need to use a method, you `______` it. 

> call

3. What symbols help to section off the code that is part of a given method?

> { }

4. Which of the following is a correct method header?

```java
_____  _____  _____  doStuff() {}
```

> public static void

5. Consider the following code:

```java
public static void doStuff() {
  int x = 82;
  if (x % 10 >= 5) {
    System.out.println("first");
  } else if (x % 5 >= 2) {
    System.out.println("second");
  } else if (x % 20 >= 10) {
    System.out.println("third");
  } else if (x % 6 == 2) {
    System.out.println("fourth");
  }
}
```

What is displayed by the call `doStuff()`?

> second