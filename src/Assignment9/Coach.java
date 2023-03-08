// package Assignment9;

public class Coach extends Person {
    public final String role;
    public  Coach(final String firstName, final String lastName, final String role) {
        super(firstName, lastName);
        this.role = role;
    }

    public final String toString() {
        return super.toString() + "\n   Role: " + this.role;
    }
}