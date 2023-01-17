1. Consider the following code segment.

```java
int[] nums = {10, 5, 8, 13};
for (int i : nums) {
  System.out.print(i + " ");
}
```

What is printed when this code segment is run?

> 10 5 8 13

2. Consider the following method, which is intended to print the first letter of every String in the array words.

```java
public static void printInitials(String[] words) {
  for (String s : words) {
    System.out.println(/* missing code */);
  }
}
```

Which of the following should replace `/* missing code */` so that the method works as intended?

> `s.substring(0, 1)`

3. For which of the following tasks would an enhanced for loop be appropriate? Select all that apply.

> Determining how many elements in an array of doubles are positive 
>
> Printing every even number in an array of ints 
>
> Printing every element in an array 

4. What is printed when the following code segment is run?

```java
int[] arr = { 3, 5, 8, 10 };
for (int n : arr) {
  n *= 2;
}

System.out.println(arr[2]);
```

> 8

5. What is printed when the following code segment is run?

```java
String[] days = { "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday" };
for (String s : days) {
  s = s.substring(0, 3);
}

System.out.println(days[0]);
```

> Monday

6. What is printed when the following code segment is run?

```java
RegularPolygon[] polygons = { new RegularPolygon(3), new RegularPolygon(4), new RegularPolygon(5) };
for (RegularPolygon p : polygons) {
  if (p.getNumSides() % 2 == 1) {
    p.addSides();
  }
}

for (RegularPolygon p : polygons) {
  System.out.println(p);
}
```

To reference the documentation for the RegularPolygon class, [click here](https://coderunner.projectstem.org/docs/shapes/index.html?_ga=2.118029760.857672948.1673974394-457991725.1660752090)

> square with side length 1.0
> 
> square with side length 1.0
> 
> hexagon with side length 1.0

7. What is printed when the following code segment is run?

```java
Rectangle[] rectangles = { new Rectangle(4, 1), new Rectangle(2, 5), new Rectangle(7, 6) };
for (Rectangle r : rectangles) {
  if (r.getPerimeter() > 12) {
    r.setLength(3);
  } else {
    r = new Rectangle(1, 2);
  }
}

for (Rectangle r : rectangles) {
  System.out.print(r.getLength() + " ");
}
```

To reference the documentation for the Rectangle class, [click here](https://coderunner.projectstem.org/docs/shapes/index.html?_ga=2.175586795.857672948.1673974394-457991725.1660752090)

> 4 3 3