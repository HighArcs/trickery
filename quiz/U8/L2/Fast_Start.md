1. Write a line of code creating a 2D array of integers called data. This array should have 6 rows and 4 columns. 

> `int[][] data = new int[6][4];`

2. What is printed when the following code segment is executed?

```java
int[][] mat = {
    {13, 8, 7, 4, 2},
    {12, 5, 6, 1, 10},
    {11, 3, 14, 9, 15}
};

System.out.println(mat[2][1]);
```

> `3`

3. What is printed when the following code segment is executed?

```java
int[][] mat = {
    {3, 18, 2, 9, 1},
    {12, 1, 6, 18, 10},
    {5, 11, 14, 9, 15}
};

for (int i = 0; i < mat.length; i += 2) {
    for (int j = 0; j < mat[i].length; j += 2) {
        System.out.print(mat[i][j] + " ");
    }
}
```

> 3 2 1 5 14 15