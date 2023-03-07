package Assignment9;

public class Person {
    public final String firstName;
    public final String lastName;

    public Person(final String firstName, final String lastName) {
        this.firstName = firstName;
        this.lastName = lastName;
    }

    public int throwDisc(final int pow) {
        if (pow < 1) {
            return 2 * 1;
        }

        if (pow > 10) {
            return 2 * 10;
        }

        return 2 * pow;
    }

    public String toString() {
        return this.lastName + ", " + this.firstName;
    }
}