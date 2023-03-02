1. Consider the array declaration that follows.

```java
int[] a = { 1, 3, 5, 6, 7, 8, 9, 11, 13, 14, 15 };
```

How many comparisons would it take if you were to search the array for 3 using a binary search? You should apply the algorithm exactly as specified in the lesson, using integer division when calculating midpoints.

> 2

2. Consider the following method headers of a binary search implementation searching for the String target in an array of String values.

```java
public static int binarySearch(String target, String[] vals) {
  return /* call to binaryHelper */;
}

/** Recursive method. 
 *  Precondition: Elements of vals are sorted in alphabetical order 
 *  @return index of target if target is in vals with an index between start and
 *  end inclusive. Returns -1 otherwise.
 */
private static int binaryHelper(String target, String[] vals, int start, int end) {
  /* implementation not shown */
}
```

Which of the following should replace `/* call to binaryHelper */` so the `binarySearch` method searches for the String `target` in the array `vals`?

> `binaryHelper(target, vals, 0, vals.length - 1)`

3. Consider again the binary search implementation:

```java
public static int binarySearch(String target, String[] vals) {
  return /* call to binaryHelper */;
}

/** Recursive method. 
 *  Precondition: Elements of vals are sorted in alphabetical order 
 *  @return index of target if target is in vals with an index between start and
 *  end inclusive. Returns -1 otherwise.
 */
private static int binaryHelper(String target, String[] vals, int start, int end) {
  /* implementation not shown */
}
```

One of the initial lines of code in the binaryHelper method calculates a mid-point index as shown:
```java
mid = (start + end) / 2;
```
Which of the following conditions could represent sensible "base cases" for the binaryHelper method under which no more recursive calls should be made to the method? Select all that apply.

> * `vals[mid].equals(target)`
> 
> * `end < start`

4. Which of the following is NOT true about binary search?

> The binary search algorithm is usually easier to code than the linear search algorithm.
