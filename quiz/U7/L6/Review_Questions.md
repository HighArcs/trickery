1. The insertion sort and selection sort are... 

> Relatively easy to code but slow to run on large datasets.

2. Consider the following implementation of the insertion sort algorithm for an ArrayList of Strings.

```java
public static int insertSort(ArrayList<String> list) {
  int count = 0;

  for (int i = 1; i < list.size(); i++) {
    String toInsert = list.get(i);
    int j;

    for (j = i; j > 0; j--) {
      count++;
      if (toInsert.compareTo(list.get(j-1)) >= 0) {
        break;
      }
    }

    list.add(j, list.remove(i));
  }

  return count;
}
```

The method returns an int which represents a statement execution count. What will this value be when the method returns if the parameter list contains the following Strings in the following order:

```java
"drive", "computer", "memory", "algorithm"
```

> 5

3. Consider the following implementation of the selection sort algorithm for an Array of doubles.

```java
public static int selectSort(double[] arr) {
  int count = 0;
  for (int i = 0; i < arr.length - 1; i++) {
    int place = i;
    for (int j = i + 1; j < arr.length; j++) {
      count++;

      if (arr[j] < arr[place]) {
        place = j;
      }
    }

    double temp = arr[i];
    arr[i] = arr[place];
    arr[place] = temp;
  }

  return count;
}
```

The method returns an int which represents a statement execution count.
What will this value be when the method returns if the parameter list is the array `{3.5, 4.8, 2.1, 4.2}`:

> 6

4. Consider the following implementation of the insertion sort algorithm for an Array of doubles.

```java
public static int insertSort(double[] arr) {
  int count = 0;
  for (int i = 1; i < arr.length; i++) {
    double val = arr[i];
    int j;

    for (j = i - 1; j >= 0; j--) {
      count++;
      if (arr[j] > val) {
        arr[j + 1] = arr[j];
      } else {
        break;
      }
    }

    arr[j + 1] = val;
  }

  return count;
}
```

The method returns an int which represents a statement execution count.
What will this value be when the method returns if the parameter list is the array `{3.5, 4.8, 2.1, 4.2}`:

> 5