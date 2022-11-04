1. Will the following statement evaluate to true or false?

Assume: A is true and B is true.

```java
!A != !B
```

> False

2. Will the following statement evaluate to true or false?

Assume: A is true and B is false.

```java
!A != !B
```

> True

3. Suppose x and y are int variables set to inputs from the user and consider the following code which is intended to check whether x is a multiple of y.

```java
if (/* missing condition */) {
  System.out.println(x + " is a multiple of " + y)
}
```

Which of the following should replace `/* missing condition */` so the code functions as intended and does not cause an error?

> `y != 0 && x % y == 0`
