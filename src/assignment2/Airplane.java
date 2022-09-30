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

        final double x1 = this.distance * Math.cos(Math.toRadians(this.direction));
        final double y1 = this.distance * Math.sin(Math.toRadians(this.direction));

        final double x2 = other.distance * Math.cos(Math.toRadians(other.direction));
        final double y2 = other.distance * Math.sin(Math.toRadians(other.direction));

        return Math.round(Math.sqrt(Math.pow(x2 - x1, 2) + Math.pow(y2 - y1, 2)) * 1000) / 1000;

        // "better"
        // return Math.sqrt(Math
        // .pow((other.distance * Math.cos(Math.toRadians(other.direction)))
        // - (this.distance * Math.cos(Math.toRadians(this.direction))), 2)
        // + Math.pow((other.distance * Math.sin(Math.toRadians(other.direction)))
        // - (this.distance * Math.sin(Math.toRadians(this.direction))), 2));
    }
}
