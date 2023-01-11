1. What does the following algorithm do?

```java
public static void mystery(int[] nums) {
  for (int i = 0; i < nums.length; i += 2) {
    nums[i] *= 2;
  }
}
```

> Doubles every other value in the array. 

2. Consider the following method.

```java
public static void insert(int[] nums, int val, int pos) {
  for (int i = nums.length - 1; i > pos; i--) {
    nums[i] = nums[i - 1];
  }

  nums[pos] = val;
}
```

What will be the contents of the array arr after the code below in the main method of the same class is run?

```java
int[] arr = {2, 3, 7, 11, 4, 8};
insert(arr, 10, 3);
```

> `{2, 3, 7, 10, 11, 4}`

3. Consider the following method.

```java
public static void delete(int[] nums, int pos) {
  for (int i = pos; i < nums.length - 1; i++) {
    nums[i] = nums[i + 1];
  }
}
```

What will be the contents of the array arr after the code below in the main method of the same class is run?

```java
int[] arr = { 7, 4, 8, 12, 1 };
delete(arr, 2);
```

> `{ 7, 4, 12, 1, 1 }`

4. The following code segment is intended to shift all Strings in the array arr one place to the right, replacing the first String in the array with the last String in the array.

```java
String[] arr = {"abc", "def", "ghi", "jkl"};
String temp = arr[arr.length - 1];

for (int i = arr.length - 2; i >= 0; i--) {
  /* missing code */
}

arr[0] = temp;
```

What should replace `/* missing code */` so that this code segment works as intended?

> `arr[i + 1] = arr[i];`

5. The following code segment is intended to reverse the order of the elements in nums.

```java
double[] nums = { 2.7, 0.3, 1.8, 9.2, 10.5, 5.4 };
for (/* missing code */) {
  int j = nums.length - i - 1; //index of element in end half
  double temp = nums[i];
  nums[i] = nums[j];
  nums[j] = temp;
}
```

What should replace `/* missing code */` so that the code works as intended?

> `int i = 0; i < nums.length / 2; i++`