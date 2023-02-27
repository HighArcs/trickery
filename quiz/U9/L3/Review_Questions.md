1. You need to store information about Fruit and Apple. Which of the following would you most agree with?

> Apple should be a subclass of Fruit.

2. You need to store information about computer systems using classes named `Hardware`, `Software` and `Ram`. Which of the following class relationships would be most sensible?

> `Ram` is a `Hardware`

3. Which option best describes the following class definition?

```java
public class Dragon extends MythicalAnimal { ... }
```

> Dragon is a MythicalAnimal.

The next two questions refer to the two classes defined below:

```java
public class Human {
    public Human() {
        System.out.print("I can run");
    }

    public void speak() {
        System.out.print("I can also talk");
    }
}

public class SuperHero extends Human {
    public SuperHero() {
        System.out.print(" and also fly");
    }

    public void speak() {
        System.out.print("I can now talk faster");
    }
}
```

4. What is printed when the following line of code is executed?

```java
Human Bob = new SuperHero();
```

> I can run and also fly

5. If example is declared as in the previous question:

```java
SuperHero superBob = new SuperHero();
```
What is printed when the following call is made?
```java
superBob.speak();
```

> I can now talk faster 