1. When you pass an object to a method, the method receives `______`. 

> a copy of the reference to the object 

2. Consider the following code, which appears in the main method of a class

```java
RegularPolygon myShape = new RegularPolygon(5, 3.5);
doStuff(myShape);
System.out.println(myShape);
```

Where `doStuff` is defined in the same class as:

```java
public static void doStuff(RegularPolygon r) {
  r.addSides(3);
  r.setSideLength(2.0);
}
```

What will be printed by the code in main?

> regular octagon with side length 2.0

3. Consider the following code:

```java
int num = 80;
doStuff(num);
```
Where `doStuff` is defined as:

```java
public static void doStuff(int b) {
  b--;
}
```

What value is stored in num after the `doStuff()` method is called and the code inside of `doStuff()` is run?

> 80

4. When a parameter which is of a mutable class type (i.e. not a String) is passed to a method in Java, `______`. 

> all changes are saved because a copy of the reference to the memory address is passed 

5. Consider the following method:

```java
public static void doStuff(int a, int b) {
  a++;
  b++;
  System.out.println(a + " " + b);
}
```

What is output by the following code?

```java
int x = 7;
int y = 5;
doStuff(x, y);
System.out.println(x + " " + y);
```
> 8 6
> 
> 7 5

6. When this type of variable is passed into a method, any changes made to it in the method are saved and the previous value of the variable is overwritten outside the method. 

> class