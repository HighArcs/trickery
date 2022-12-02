// @ignore Java(536871240)
// package U5_L5_Activity_One;

public class Person {
    final String firstName;
    final String lastName;
    final int age;
    final String ssn;

    public Person(String firstName, String lastName, int age, String ssn) {
        this.firstName = firstName;
        this.lastName = lastName;
        this.age = age;
        this.ssn = ssn;
    }

    public String toString() {
        return "SSN: " + this.ssn + "\n\tName: " + this.firstName + " " + this.lastName + "\n\tAge: " + this.age;
    }
}
