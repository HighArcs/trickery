// package Assignment9;

public class Captain extends UltimatePlayer {
    public final boolean type;

    public Captain(final String firstName, final String lastName, final String position, final boolean type) {
        super(firstName, lastName, position);
        this.type = type;
    }

    public final int throwDisc(final int pow) {
        return (super.throwDisc(pow) / 4) * 5;
    }

    public final String toString() {
        String text = "defense";
        if (this.type) {
            text = "offense";
        }
        return super.toString() + "\n   Captain: " + text;
    }
}