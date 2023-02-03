1. Consider the following array and method declarations.

```java
int[] list = { 1, 3, 5, 6, 7, 8, 11, 13, 14, 15, 16, 17, 20, 22, 24, 29, 30, 31, 33, 35 };

public static int mystery(int[] a, int s) {
  for (int i = 0; i < a.length; i++) {
    if (a[i] == s) {
      return i;
    }
  }

  return -1;
}
```


What would be returned by the following method call?

```java
mystery(list, 17)
```

2. Which of the following could be used to measure the efficiency of an algorithm which has been coded?

> Running a statement execution count 

3. Which type of algorithm is performed on the array arr by this code?

```java
double[] arr = //initial values set here
int a = //initial value set here
for (int i = a; i < arr.length-1; i++) {
  arr[i] = arr[i + 1];
}
```

> Item removal