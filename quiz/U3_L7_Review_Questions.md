1. Which of the following is true about the equals method? 

> The way it works depends on the class of the object
> 
> It is used to check if two objects are equal
> 
> It exists in all classes in Java

2. What is displayed when the following code is compiled then run?

```java
String s1 = "hello";
String s2 = "Hello";
if (s1.equals(s2)) {
  System.out.println("yes");
} else {
  System.out.println("no");
}
```

> no

3. What is displayed when the following code is run?

```java
String s1 = new String("Java");
String s2 = new String("Java");
String s3 = s2;

if (s1 == s2) {
  System.out.print("y");
} else {
  System.out.print("n");
}

if (s1 == s3) {
  System.out.print("y");
} else {
  System.out.print("n");
}

if (s2 == s3) {
  System.out.print("y");
} else {
  System.out.print("n");
}
```

> nny

4. What is displayed when the following code is compiled then run?

```java
int a = 7;
int b = 7;
if (a.equals(b)) {
  System.out.println("yes");
} else {
  System.out.println("no");
}
```

> Nothing is displayed. An error stops the code from compiling.

5. What is printed when the following code is run?

```java
String str = "thinking";
String start = str.substring(0, 2);
String end = str.substring(str.length() - 3);

if (start.equals("th") && end.equals("ing")) {
  System.out.println("Test 1 passed");
}
if (!str.equals("thing")) {
  System.out.println("Test 2 passed");
}
```

> Test 1 passed
>
> Test 2 passed