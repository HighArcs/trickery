package assignment2;

public class Airplane {
    private String callSign;
    private double distance;
    private int direction;
    private int altitude;

    public Airplane() {
        this("AAA01", 1.0f, 0, 0);
    }

    public Airplane(String callSign, double distance, int direction, int altitude) {
        this.callSign = callSign;
        this.distance = Math.abs(distance);
        this.direction = direction % 360;
        this.altitude = Math.abs(altitude);
    }

    public void move(double dist, int dir) {
        this.distance += dist;
        this.direction = dir;
    }

    public void gainAlt() {
        this.altitude += 1000;
    }

    public void loseAlt() {
        this.altitude -= Math.min(1000, this.altitude);
    }

    public int getAlt() {
        return this.altitude;
    }

    public String toString() {
        return String.format("%s - %.1f miles away at bearing %03d°, altitude %d feet", this.callSign, this.distance,
                this.direction, this.altitude);
    }

    public double distTo(Airplane other) {
        int t = Math.abs(this.direction - other.direction);
        if (t >= 180) {
            t = 360 - t;
        }

        // c^2 = a^2 + b^2 - 2ab * cos(t)

        double a = this.distance, b = other.distance;
        double ct = Math.cos(Math.toRadians(t));

        double a2 = Math.pow(a, 2);
        double b2 = Math.pow(a, 2);
        double ab2cos = 2 * a * b * ct;

        // truncate to 2 decimal places
        return Math.sqrt(a2 + b2 - ab2cos);
    }
}
