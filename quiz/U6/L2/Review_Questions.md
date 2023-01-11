1. Consider the following code:

```java
double [] vals = { 1.8, 3.4, 7.2, 4.4, 0.3, 2.9, 1.1, 9.5, 6.2, 0.8, 2.4, 5.7 };
Scanner scan = new Scanner(System.in);
double what = scan.nextDouble();
int location = -1;

for (int i = 0; i < vals.length; i++) {
  if (what == vals[i]) {
    location = i;
    break;
  }
}

System.out.println(location);
```

What will be output if the user enters `4.4`?

> 3

2. Consider the algorithm in the following method:

```java
public static int algorithm(String[] names, String lookingFor) {  
  int found = -1;
  for (int i = 0; i < names.length; i++) {
    if (lookingFor.equals(names[i])) {
      found = i;
      break;
    }
  }
  return found;
}
```

Which of the following best describes what this algorithm does?

> Finds the first location of the name the user enters, -1 if not found. 

3. The following is intended to count the number of times the number 100 is found in an array of test scores:

```java
int[] d = { 3, 22, 100, 88, 25, 100, 72, 99, 88 };
int perfect = 0;
for (int i = 0; i < d.length; i++) {
  if (/* missing code */) {
    perfect++;
  }
}

System.out.println("Number of perfect scores: " + perfect);
```

Which of the following could replace `/* missing code */` so that the code works as intended?

> `d[i] == 100`

4. The following linear search method looks for the int value stored in the array d:

```java
public static void search(int[] d, int value) {
  int flag = -1;
  for (int i = 0; i < d.length; i++) {
    if (d[i] == value) {
      flag = 1;
    }
  }

  if (flag == 1) {
    System.out.println("Value found");
  } else {
    System.out.println("Value not found");
  }
}
```

Why is flag initialized to -1?

> It assumes the value will not be found. -1 indicates it is not found. 

