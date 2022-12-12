1. Which of the following is true about static variables? 

> Static variables are shared among all instances of a class. 

2. Consider the following class.

```java
public class ClassicClass {
  private static int count = 0;
  private int num;

  public ClassicClass() {
    count++;
    num = count;
  }

  public String toString() {
    return count + " " + num;
  }
}
```

What is printed when the following code in the main method of another class is run?

```java
ClassicClass a = new ClassicClass();
ClassicClass b = new ClassicClass();
ClassicClass c = new ClassicClass();
System.out.println(a + ", " + b + ", " + c);
```

> 3 1, 3 2, 3 3

3. Here is a description of variables used in a gas station class. 

```
customer - represents the name of the person buying gas
vehicle - represents the type of vehicle of the person buying gas
gasPrice - represents the cost per gallon of gas on one specific day
customerBill - represents the total amount of money owed by person buying gas
```

Based on these descriptions, which of the following variables would be static?

> gasPrice

4. You are writing a class for a chain of businesses with the same hours. Which of the following would you want to make static? 

> A `boolean` to determine if the stores are opened or closed. 

5. The following 2 questions refer to the class favNums which is defined below:

```java
public class FavNums {
  private int num1;
  private int num2;
  private int num3;

  public FavNums(int n1, int n2, int n3) {
    num1 = n1;
    num2 = n2;
    num3 = n3;
  }

  public void printFavorites() {
    System.out.println(num1 + num2 + num3);
  }

  public static void printRandoms() {
    int r1 = (int)(100 * Math.random());
    int r2 = (int)(100 * Math.random());
    int r2 = (int)(100 * Math.random());
    System.out.println(r1 + r2 + r3);
  }
}
```

Which of the following code segments show correct calls to the method `printFavorites()` which would compile and run in a method of a separate class?

> ```java
> FavNums f = new FavNums(1, 2, 3);
> f.printFavorites();
> ```

6. Which of the following code segments show correct calls to the method `printRandoms()` which would compile and run in a method of a separate class?

There may be more than one answer to this question - choose all that apply.

> ```java
> FavNums f = new FavNums(1, 2, 3);
> f.printRandoms();
> ```
>
> ```java
> FavNums.printRandoms();
> ```
