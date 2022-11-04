1. What are the first and last numbers printed by the following code:

```java
int x = 20;
while (x <= 25) {
  x++;
  System.out.print(x + " ");
}
```

> 21 26

2. Consider the following code:

```java
int n = 75;
while (n > 0) {
  System.out.print(n + " ");
  n /= 10; 
}
```

What is output?

> 75 7

3. What does the following loop do:

```java
int num = scan.nextInt();
int sum = 0;

while (num > 0) {
  sum += num % 10;
  num /= 10;
}
System.out.print(sum);
```

> Sums each digit of the number the user enters. 

4. What is the difference between the two loops?

I. 
```java
int num = 0;
int c = 0;
while (num != 100) {
  num = scan.nextInt();
  c++;
}
```

II.

```java
int num = 0;
int c = 0;
while (c != 100) {
  num = scan.nextInt();
  c++;
}
```

> I stops by using user input, II stops by using a count variable. 

5. Consider the following code, assume a and b are integers and have been initialized.

What is output when a = 55 and b = 45?

```java
int f = 0;
int d = 2;
while (d <= a) {
  if (a % d == 0 && b % d == 0) {
    f = 1;
  }

  d++;
}

System.out.println(f);
```

> 1

6. Consider the following code, assume a and b are integers and have been initialized.

What is output when a = 66 and b = 25?

```java
int f = 0;
int d = 2;
while (d <= a) {
  if (a % d == 0 && b % d == 0) {
    f = 1;
  }

  d++;
}

System.out.println(f);
```

> 0