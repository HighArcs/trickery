1. Assume you have the following array:

```java
int[] a = { 1, 2, 3, 4, 5 };
```

How would you increment the third element in the array by one?

> a[2]++;

2. Consider the following code:

```java
double[] list = new double[100];
```

The index of the first value is `______` and the last index is `______`.

> 0, 99

3. A standard array can hold:

> Either class or primitive types.

4. Consider the following code:

```java
int[] a = { 2, 6, 8, 10, 12, 14, 16, 18 };

int sum = 0;
for (int i = 0; i < a.length; i++) {
  if (i % 3 == 0) {
    sum += a[i];
  }
}

System.out.println(sum);
```

What is output?

> 28

5. Consider the following code, intended to search an array for a value and print the position where that value was found:

```java
int[] array = /* Assume array is correctly initialized */;
int num = /* Input from the keyboard */;
int position = -1;

for (int i = 0; i < array.length; i++) {
  if (array[i] == num) {
    /* Missing Code */
    break;
  }
}

if (position == -1) {
  System.out.println("Value not found");
} else {
  System.out.println("Value found at position " + position);
}
```

What could replace `/* Missing Code */` so that the code works as intended?

> `position = i;`

6. The following is intended to count the number of times the number 87 is found in an array of test scores:

```java
int[] d = /* Assume array is initialized */;
int scoreCount = 0;
for (int i = 0; i < d.length; i++) {
  if (/* Missing Code */) {
    scoreCount++;
  }
}

System.out.println("Number of 87's: " + scoreCount);
```

> 26

Which of the following could replace `/* Missing Code */` so that the code works as intended?

> `d[i] == 87`

7. Consider the following code, intended to count the number of words in the array with a length less than or equal to 6.

```java
String[] vocabulary = /* Array initialized with Strings */;
int c = 0;

for (int i = 0; i < vocabulary.length; i++) {
  if (/* Missing Code */) {
    c++;
  }
}

System.out.println("Number of words with length less than or equal to 6: " + c);
```

What could be used to replace `/* Missing Code */` so that the code works as intended?

> `vocabulary[i].length() <= 6`

8. Consider the following code:

```java
String[] words = {"avalanche", "budget", "cannot", "center", "meaning", "clear", "furniture", "deep", "piccolo", "friendly", "potatoes", "nanotechnology"};
int c = 0;
for (int i = 0; i < words.length; i++) { 
  if (words[i].substring(0, 3).indexOf("o") >= 0) {
    c++;
  }
}

System.out.println(c);
```

What is output?

> 1

9. The following is intended to return the location of the first instance of the String the user enters from the keyboard, -1 if not found.

```java
String[] names = new String[20];
//assume array is initialized

System.out.println("Enter a name to search for: ");
String lookingFor = scan.nextLine();

int found = -1;

for (int i = 0; i < names.length; i++) {
  if (/* Missing Code */) {
    found = i;
    break;
  }
}
```

Which of the following could replace `/* Missing Code */` so that it works as intended?

> `lookingFor.equals(names[i])`

10. Consider the following method, hasRepeats, which is intended to return true if an array of integers contains the same value more than once and false otherwise.

```java
/** @param arr an array of integers
 *  @return true  if an element in the array is repeated
 */
public static boolean hasRepeats(int[] arr) {
  for (int k = 0; k < arr.length; k++) {
    for (/* missing code */) {
      if (arr[j] == arr[k]) {
        return true;
      }
    }
  }
  return false;
}
```

Which of the following should be used to replace `/* missing code */` so that hasRepeats works as intended?

> `int j = k + 1; j < arr.length; j++`

