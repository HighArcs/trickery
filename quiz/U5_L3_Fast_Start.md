1. What key word shows that a method will not return a value? 

> void

2. Consider the method defined below:

```java
public static void answerPhone() {
  System.out.println("Ahoy-hoy!");
}
```

How would you call this method from main?

> answerPhone();

3. Looking at the following method:

```java
public static void repeatPrint(String s, int n) {
  for (int i = 0; i < n; i++) {
    System.out.print(s + " ");
  }
}
```

Which of the parameters are class-type, and which are primitive?

> s is class-type, n is primitive 

4. Again, looking at the same method as above:

```java
public static void repeatPrint(String s, int n) {
  for (int i = 0; i < n; i++) {
    System.out.print(s + " ");
  }
}
```

What is printed when the following call is made from main?

```java
repeatPrint("done", 3);
```

> `done done done`