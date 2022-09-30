package assignment2;

public class Airplane {
    private double distance;
    private int direction;
    private int altitude;
    private String callSign;

    public Airplane(String cs, double dist, int dir, int alt) {
        this.distance = Math.abs(dist);
        this.direction = dir % 360;
        this.altitude = Math.abs(alt);
        this.callSign = cs;
    }

    public Airplane() {
        this("AAA01", 1, 0, 0);
    }

    public void gainAlt() {
        this.altitude += 1000;
    }

    public void loseAlt() {
        this.altitude -= Math.min(this.altitude, 1000);
    }

    public int getAlt() {
        return this.altitude;
    }

    public void move(double dist, int dir) {
        final double  r1 = this.distance;
        final double  r2 = dist;

        final double  u1 = Math.toRadians(direction);
        final double  u2 = Math.toRadians(dir);

        final double r1s = Math.pow(r1, 2);
        final double r2s = Math.pow(r2, 2);
        final double r2c = 2 * r1 * r2 * Math.cos(u2 - u1);

           this.distance = Math.sqrt(r1s + r2s + r2c);

        final double  y1 = r1 * Math.sin(u1);
        final double  y2 = r2 * Math.sin(u2);
        final double  x1 = r1 * Math.cos(u1);
        final double  x2 = r2 * Math.cos(u2);

        final double ang = Math.atan2(y1 + y2, x1 + x2);

          this.direction = Math.floorMod((int) Math.round(Math.toDegrees(ang)), 360);
    }

    public String toString() {
        return String.format("%s - %.2f miles away at bearing %03d\u00b0, altitude %d feet", this.callSign,
                this.distance, this.direction, this.altitude);
    }

    public double distTo(Airplane other) {

        double r1 = this.distance;
        double r2 = other.distance;

        double u1 = Math.toRadians(this.direction);
        double u2 = Math.toRadians(other.direction);

        double r1s = Math.pow(r1, 2);
        double r2s = Math.pow(r2, 2);
        double r2c = 2 * r1 * r2 * Math.cos(u2 - u1);

        double between = Math.sqrt(r1s + r2s - r2c);

        return (double) Math.round(100 * between) / 100;
    }
}
