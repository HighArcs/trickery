1. Which of the following would you use to change the element at position `i` of an array named nums to the element at position `j` of an array named `moreNums`?

> `nums[i] = moreNums[j]`

2. Which of the following gives the loop header needed to traverse each of the elements in the array map? 

> `for (int i = 0; i < arr.length; i++)`

3. Consider the following method.

```java
public static String findA(String[] arr) {
    for (int i = 0; i < arr.length; i++) {
        if (arr[i].substring(0, 1).equals("g")) {
            return arr[i];
        }
    }

    return "";
}
```

Suppose the array words is initialized as follows:

```java
String[] words = {"remain", "believe", "argue", "antagonize"};
```

What is returned by the call `findA(words)`?

> `"argue"`

