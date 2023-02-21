1. Which of the following is true about overloading and overriding methods?

> When a child class has a local version of a method with the same return type that is defined in a parent class this is called overriding.

2. Consider the following class declarations.

```java
public class Instrument {

    public Instrument() {
        System.out.println("play music");
    }

    public void play(String key) {
        System.out.println("The root note starts in " + key + “ note”);
    }
}

public class Piano extends Instrument {

    public Piano() {
        super();
    }

    public void play(int octave) {
        System.out.println("Start playing in the " + octave + "th octave");
    }

}
```


Piano shows an example of:

> Overloading only

3. Consider the following method from the class Instrument.

```java
public void play() {
    System.out.println("play music");
}
```

Which of the following methods implemented in `Piano`, a child class of `Instrument` correctly overrides the method speak?

> II and III