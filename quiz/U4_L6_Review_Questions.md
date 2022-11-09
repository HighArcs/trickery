1. Adding a tracking variable to count the number of times a loop executes can be an effective way to measure efficiency.

> True

2. Consider the following algorithm acting on a String str.

```java
int count = 0;
 for (int i = 0; i < str.length(); i++)  {
   for (int j = i; j >= 0; j--) {
     String a = str.substring(j, j + 1);
     if (a.equals(str.substring(i, i + 1))) {
       System.out.print(a);
     }
   }
}
```

Suppose we wish to use the variable count to measure the number of times two characters in the string are compared. What line of code should be added and where?

>  Add the line count++; between lines 5 and 6. 

3. How many times will line 6 be executed when the following code is run?

```java
for (int i = 0; i <= 15; i += 5) {
    int s = 0;
    for (int j = i; j < i + 3; j++) {
        s += j;
    }

    System.out.print(s + " ");
}
```

> 12

4. How many times will line 7 be executed when the following code is run?

```java
String str = "rebellion";
int i = 0;
int j = str.length() - 1;
String result = "";

while (i < j) {
  result = str.substring(i, i + 1) + result + str.substring(j, j + 1);
  i++;
  j--;
}

System.out.println(result);
```

> 4

5. Consider the following two algorithms which both are meant to print all multiples of 11 from 1 up to a user input positive integer value - upper. Which statement correctly compares the efficiency of these two algorithms?

Algorithm 1:

```java
for (int i = 1; i <= upper; i++) {  
  if (i % 11 == 0) {    
    System.out.println(i + " ");  
  }
}
```

Algorithm 2:

```java
for (int i = 1; i <= upper / 11; i++) {  
  System.out.println(i * 11 + " ");
}
```

> Algorithm 2 is more efficient for all possible values of upper.
