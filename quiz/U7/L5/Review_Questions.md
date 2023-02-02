1. The following code is intended to sort an input array of integers into ascending order.

```java
for (int j = a.length - 1; j > 0; j--) {
  int pos = j;
  for (/* missing code */) {
    if (a[k] > a[pos]) {
      pos = k;
    }
  }
  swap(a, j, pos);
}
```

Assume `swap(a, j, pos)` swaps the values of units `j` and `pos` in array `a`. Which of the following code segments, if replaced for `/* missing code */`, will make the code run properly?

> `int k = j - 1; k >= 0; k--`

2. The array arr is declared and initialized as shown:

```java
int[] arr = { 7, 2, 13, 4, 11, 3 }
//            2, 7, 13, 4, 11, 3
//            2, 3, 13, 4, 11, 7
//            2, 3, 4, 13, 11, 7
```

If this array is sorted using the selection sort algorithm, what will the array look like after three swaps have been performed in this algorithm?

> `{ 2, 3, 4, 13, 11, 7 }`

3. Consider the following implementation of the selection sort intended to sort the ArrayList of String objects list into (forward) alphabetical order:

```java
public static void sort(ArrayList<String> list) {
  for (int i = 0; i < list.size() - 1; i++) {
    int low = i;
    for(int j = i + 1; j < list.size(); j++) {
      if (/* missing condition */) {
        low = j;
      }
    }

    String temp = list.get(i);
    list.set(i, list.get(low));
    list.set(low, temp);
  }
}
```

Which of the following should replace `/* missing condition */` so this method works as intended?

> `list.get(j).compareTo(list.get(low)) < 0`

4. Why can't we use a for-each loop to implement a selection sort on an ArrayList?

> We need the index number of each element to know which values to compare them to and also to swap the values. We do not get the index number with a for-each loop.
