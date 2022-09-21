1. What keyword do we use to tell Java to set aside memory to create an object? 

> new

2. Which of these code segments shows a constructor being used to create a new object?

> ```java
> RegularPolygon p = new RegularPolygon(2, 2.5);
> ```

3. Which of the following creates a rectangle with length and width equal to 2.0?

Choose all answers that apply.

> ```java
> Rectangle r = new Rectangle(2.0, 2.0);
> ```
> 
> ```java
> Rectangle r = new Rectangle(2.0, 2.0, 2.0, 2.0);
> ```

4. Which of the following will be printed by the following code?

```java
RegularPolygon shape = new RegularPolygon(5.0);
System.out.println(shape);
```

> equilateral triangle with side length 5.0

5. Which of the following statements about overloaded constructors is true? 

> Constructors for a class can be overloaded but they must all have a different number of parameters, different order of parameters or different types of parameters.

6. Which of the following best describes what happens when the code segment below is run?

```java
RegularPolygon p = new RegularPolygon();
System.out.println(p);
```

> "equilateral triangle with side length 1.0" is printed

