1. Consider the following declaration

```java
Lego newBlock = new Lego();
```

Which of the following best describes the situation?

> newBlock is an object of the Lego class type

2. Which of the following is used to indicate a new comment? 

> `//`

3. Assume that Bank is a class that creates a checking account object. By default, the money stored will be 100 dollars. Also assume that withdrawMoney() is a void method that will update the balance by subtracting 50 dollars to the current money stored.

```java
Bank checkingAccount1 = new Bank();
Bank checkingAccount2 = checkingAccount1;

checkingAccount1.withdrawMoney();
```

What is true about checkingAccount1 and checkingAccount2?

> The value of both checkingAccount1 and checkingAccount2 loses the 50 dollars because checkingAccount2 and checkingAccount1 both point to the same object

4. What is the output of the following code segment?

```java
int y = 15;
int x = 5;
System.out.println("Answer: " + (x + 10) + y);
```

> Answer: 1515

5. Consider the following code:

```java
String str = "Java";
```

Which of the following statements correctly prints the third character in the String str?

> System.out.println(str.substring(2, 3));

6. The class Computer contains a non-static void method named troubleShoot with a parameter that contains one String argument. Suppose a Computer object mac has been declared as follows:

```
Computer mac = new Computer();
```

Which of the following correctly can call the method troubleShoot?

> `mac.troubleShoot("on");`

7. Which of the following could be the signature of a constructor from a class named Subject?

> `Subject(String x);`

8. What is stored in the variable num after the following code is executed?

```java
int num = Math.pow(3,2) + 2;
```

> Error: possible loss of precision

9. What is printed by the following code?

```java
String a = "wallet";
String b = a;
a = b.toUpperCase();
System.out.println(b + a);
```

> walletWALLET

10. Which of the following is a call to a static method? 

> Math.sqrt()

11. What is printed by the following code segment?

```java
Integer x = 1500;
int y = x + 50;
System.out.println(Math.max(x, y));
```

> 1550

12. Consider the following code:

```java
double x = -67.6;
System.out.println(Math.abs(x)-5);
```

What is the output?

> 62.6

13. Suppose a method has a return type of Double, the name firstMethod and a parameter list of int num, String word (in that order). Which of the following methods described below has the same signature as this method?

>  return type: `int`, name: `firstMethod`, parameter list: `int num, String word`

14. What is output by the following code segment?

```java
String a = "java";
String b = "guava";
System.out.println(a.compareTo(b));
```

> 3

15. Which of the following describes the return type and parameters of the Integer method compare?

> Return type: int. Parameters: two Integer parameter

16. Which of the following correctly gives random numbers between -5 and 0 inclusive?

> `int n = (int) (Math.random() * 6) - 5;`

17. The image below shows the Javadoc for all constructors of a class named Shoe.

![image](https://user-images.githubusercontent.com/73959934/194371608-2ff6c4b1-e0e2-4b00-a256-343e99d86e9f.png)

Which of the following constructor calls will NOT compile correctly?

> Shoe Nikes = new Shoe(“11”, true);

18. Consider the following code:

```java
String language1 = "java";
String language2 = "javascript";
System.out.println(language2.indexOf(language1.substring(1, 4)));
```

What is output?

> 1

19. What is output? 

```java
System.out.println("The answer is: " + (double)(5 * 6));
```

> The answer is: 30.0

20. Consider the following code: 

```java
int x = 3;
int y = 2;
System.out.println((x + y) * x);
```

What is output when this code is executed?

> 15
