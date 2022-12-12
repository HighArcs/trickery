1. Consider the following javadoc comments for a method named recase.

```java
/** 
 *  preconditions: str contains more characters than n
 *  @param str a String
 *  @param n an int less than str.length()
 *  @return a String with the first n characters of str converted to
 *  upper case, and the remaining characters converted to lower case
 */
```

Which of the following method declarations should this belong to?

> ```java
> public String recase(String str, int n) {
>  return str.substring(0, n).toUpperCase() + str.subString(n).toLowerCase();
> }
> ```

2. Consider the following method.

```java
public static int getOnes(int x) {
    return Math.abs(x) % 10;
}
```

Which of the following javadoc comments should be used to correctly generate the API for this method?

> ```java
> /**
>  * @param x an int value
>  * @return an int with the ones digit of x
>  */
> ```

3. Static methods from another class are called...

> Directly from the class