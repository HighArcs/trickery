1. Consider the following definition for an array of integers.

```java
int[] a = { 3, 4, 6463, 4, 7, 8, 5, 3, 2, 3, 6 };
```


If you were to do a linear search for 7, how many comparisons would be made? You may assume the linear search algorithm stops comparing values when it finds the correct number.

> 5

2. The following search method is intended to implement a linear search algorithm to find the String target in the ArrayList of String values searchList.

```java
public static int search(String target, ArrayList<String> searchList) {
  for (int i = 0; i < searchList.size(); i++) {
    if (/* missing condition */) {
      return i;
    }
  }

  return -1;
}
```

Which of the following could replace `/* missing condition */` so this method correctly implements a linear search algorithm?

> `searchList.get(i).equals(target)`

3. The method altSearch below is intended to implement a modified version of the linear search algorithm on an ArrayList of Integers someList. The method should return the index at which the Integer a last appears.

```java
public static int altSearch(Integer a, ArrayList<Integer> someList) {
  for (/* missing header */) {
    if (a.equals(someList.get(i))) {
      return i;
    }
  }
  return -1;
}
```

Which of the following should replace `/* missing header */` so this method works as intended?

> `int i = someList.size() - 1; i >= 0; i--`
