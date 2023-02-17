1. Consider the following code segment.

```java
int[][] array = new int[5][5];
int u = 1;
for (int i = 0; i < array.length; i++) {
  for (int j = 0; j < array[i].length; j++) {
    array[i][j] = u;
    u++;
  }
}

System.out.println(array[3][3]);
```

What is printed when this code segment is executed?

> 19

2. Suppose data is a 2-d array of Strings that has 5 rows and 5 columns, with initial contents represented by the following table:

|   |   |   |   |   |
|---|---|---|---|---|
| `O` | `O` | `X` | `O` | `O` |
| `O` | `O` | `X` | `X` | `X` |
| `O` | `O` | `X` | `O` | `O` |
| `O` | `O` | `X` | `X` | `X` |
| `O` | `O` | `X` | `O` | `O` |

Consider the following code segment.

```java
for (int i = 0; i < data.length; i++) {
  for (int j = 0; j < data[i].length; j++) {
    if (data[i][j].equals("O")) {
      data[i][j] = "X";
    } else if (data[i][j].equals("X")) {
      data[i][j] = "O";
    }
  }
}

for (String[] r : data) {
  for (String s : r) {
    System.out.print(s);
  }

  System.out.println();
}
```

Which of the following represents what is printed after the code segment is executed?

> ```
> XXOXX
> XXOOO
> XXOXX
> XXOOO
> XXOXX
> ```

3. Consider the following method which is intended to return a 1-d array containing the averages of each row in the 2-d array mat.

```java
public static double[] getAverages(double[][] mat) {
  double[] avgs = new double[mat.length];
  for (int i = 0; i < mat.length; i++) {
    /* missing code */
  }

  return avgs;
}
```

Which of the following should replace `/* missing code */` if the method is to work as intended?

> ```java
> double s = 0;
> for (double n : mat[i]) {
>   s += n;
> }
> avgs[i] = s / mat[i].length;
> ```

4. Consider the following method.

```java
public static int search(int[][] data, int target) {
  for (int i = 0; i < data.length; i++) {
    for (int j = 0; j < data[i].length; j++) {
      if (data[i][j] == target) {
        return i;
      }
    }
  }

  return -1;
}
```

The following code segment appears in the main method of the same class. What is printed when this code is executed?

```java
int[][] mat = {
    {1, 0, 4, 2, 4},
    {3, 8, 2, 4, 7}
};

System.out.println(search(mat, 4));
```

> 0