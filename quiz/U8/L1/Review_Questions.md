1. Consider the following code segment.

```java
int[][] a = new int[7][5];

for (int r = 0; r < a.length; r++) {
  for (int c = 0; /* expression */; c++) {
    a[r][c] = 5;
  }
}
```

What should replace `/* expression */` so that a 5 is stored in every cell in the array?

> `c < a[r].length`

2.  Which loop will correctly print the elements of the 2-d array of ints named arr in Column-Major order? 

> ```java
> for (int i = 0; i < arr[0].length; i++) {
>   for (int j = 0; j < arr.length; j++) {
>     System.out.print(arr[j][i] + " ");
>   }
> 
>   System.out.println();
> }
> ```

3. Which of the following statements correctly declares and initializes a 2-d array of doubles with 3 rows and 4 columns? 

> `double[][] mat = new double[3][4];`

4. Consider the following 2-d array declaration and initialization:

```java
int[][] mat = {
    { 18, 17, 16 },
    { 15, 14, 13 },
    { 12, 11, 10 },
    { 9, 8, 7 },
    { 6, 5, 4 },
    { 3, 2, 1 }
};
```

Which of the following would reference the element with value 5 in the 2-d array mat?

> `mat[4][1]`

5. Suppose we wish to traverse a 2-d array of strings named grid with two for-each loops. Which of the following represents the loop headers we should use? 

> ```java
> for (String[] row : grid) {
>     for(String str : row) {
>       // code..
>     }
> }
> ```

