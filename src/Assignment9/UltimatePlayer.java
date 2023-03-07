package Assignment9;

public class UltimatePlayer extends Person {
    public final int jerseyNumber;
    public final String position;

    private static int count = 0;

    public UltimatePlayer(final String firstName, final String lastName, final String position) {
        super(firstName, lastName);

        this.position = position;

        UltimatePlayer.count++;
        this.jerseyNumber = UltimatePlayer.count;
    }

    public final String getPosition() {
        return this.position;
    }

    public int throwDisc(final int pow) {
        // super.throwDisc() returns `pow * 2`. `2 * 2 * pow` is the same as `4 * pow`.
        return 2 * super.throwDisc(pow);
    }

    public String toString() {
        return super.toString() + "\n   Jersey #: " + this.jerseyNumber + "\n   Position: " + this.position;
    }
}