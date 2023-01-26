1. You need to process an ArrayList and remove several elements in the list. Which loop is most appropriate? 

> for loop

2. Consider the following code:

```java
ArrayList<String> stuff = new ArrayList<String>();

stuff.add("keyboard");
stuff.add("mouse");
stuff.add("speakers");
stuff.add("scanner");
stuff.add("screen");
stuff.add("headphones");

for (String s: stuff) {
  if (s.compareTo("m") > 0) {
    System.out.println(s.toUpperCase());
  }
}
```

What is output?

> ```
> MOUSE
> SPEAKERS
> SCANNER
> SCREEN
> ```

3. Consider the following instance variable and method

```java
private ArrayList<String> wordList;

public String mystery() {
  String s = "";
  for (int i = wordList.size() - 1; i >= 0; i--) {
    s += wordList.get(i).substring(0,1);
  }
  return s;
}
```

If wordList is initialized containing the values "ivory", "summer", "watch", "slam", "wolf" in that order, what String is returned by a call to the method mystery?

> wswsi

4. What is printed when the following code, using the shapes.Rectangle class, is executed?

```java
ArrayList<Rectangle> shapeList = new ArrayList<Rectangle>();
shapeList.add(new Rectangle(3, 7));
shapeList.add(new Rectangle(6, 2));
shapeList.add(new Rectangle(8, 3));
shapeList.add(new Rectangle(2, 8));
for (Rectangle r : shapeList) {
  if (r.getArea() > 20) {
    System.out.print(r.getPerimeter() + " ");
  }
}
```

> 20.0 22.0

5. Consider the following method:

```java
public static void mystery(ArrayList<Integer> list1, ArrayList<Integer> list2) {
  for (int i = 0; i < list1.size(); i++) {
    if (list1.get(i) > list2.get(i)) {
      System.out.print(list1.get(i));
    } else {
      System.out.print(list2.get(i));
    }

    System.out.print(" ");
  }
}
```

The following code appears in the main method of the same class. What is printed when this code is executed?

```java
ArrayList<Integer> nums1 = new ArrayList<Integer>();
nums1.add(7);
nums1.add(4);
nums1.add(5);
nums1.add(12);

ArrayList<Integer> nums2 = new ArrayList<Integer>();
nums2.add(5);
nums2.add(6);
nums2.add(2);
nums2.add(11);

mystery(nums1, nums2);
```

> 7 6 5 12