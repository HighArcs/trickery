1. What form of data is the binary search algorithm used on?

> `object`

2. Consider the following method:

```java
public static void mystery(int a) {
    if (a % 5 != 0) {
        mystery (a + 1);
    }


    System.out.print(a + " ");
}
```

What is output by `mystery(2)`?

> 5 4 3 2

3. Consider the following implementation of the selection sort algorithm on an array of ints named elements:
```java
for (int j = 0; j < elements.length - 1; j++) {
    int minIndex = j;
    for (int k = j + 1; k < elements.length; k++) {
        if (elements[k] < elements[minIndex]) { // comparison performed
            minIndex = k;
        }
    }

    int temp = elements[j];
    elements[j] = elements[minIndex];
    elements[minIndex] = temp;
}
```


Suppose elements is initialized as the array `{ 3, 4, 6, 2, 7, 7 }`. How many times would the line labeled "comparison performed" be executed when the code segment above is run?

> 15