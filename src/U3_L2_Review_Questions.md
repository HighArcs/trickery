1. What is the symbol for "not equal to" in Java?

> !=

2. What is the symbol for "less than or equal to" in Java? 

> <=

3. Which code tests if the number in the variable num1 is greater than 15? 

> `if (num1 > 15)`

4. The following code is intended to test if the exponent is 8 or less. You may assume that scan is a properly initialized Scanner object.

```java
int base = 2;
int exponent = scan.nextInt();
int answer = (int) Math.pow(base, exponent);
if (answer <= 256) {
  System.out.println("exponent is 8 or less");
}
```

What is the problem with the code?

> Nothing is wrong, the code works as intended.

5. Consider again the code from the previous problem:

```java
int base = 2;
int exponent = scan.nextInt();
int answer = (int) Math.pow(base, exponent);
if (answer <= 256) {
  System.out.println("exponent is 8 or less");
}
```

The if statement from this code could also be written as which of the following without ever changing the output when this code segment runs?

I.

```java
if (exponent < 8) {
  System.out.println("exponent is 8 or less");
}
```

II.

```java
if (exponent > 8) {
  System.out.println("exponent is 8 or less");
}
```

III.

```java
if (exponent <= 8) {
  System.out.println("exponent is 8 or less");
}
```

> III

6. Which of the following if statements correctly executes all three following commands only when the if condition is true?

I.

```java
if ( x < 10)
  int ans;
  ans = x * 90;
  System.out.println("answer: " + ans);
```


II.

```java
if ( x < 10)
{
  int ans;
  ans = x * 90;
  System.out.println("answer: " + ans); 
}
```

III.

```java
if { ( x < 10)
  int ans;
  ans = x * 90;
  System.out.println("answer: " + ans);
}
```

> II only

7. Water freezes at or below 32 degrees F.

Consider the following code intended to test when rain might switch over to snow. You may assume that scan is a properly initialized Scanner object.

```java
int temp = scan.nextInt();
if (/* Missing Code */) {
  System.out.println("It may snow");
}
```

The code segment should print "It may snow" whenever the value entered by the user is 32 or below. Which of the following could be used to replace `/* Missing Code*/`?

> `temp < 33`

